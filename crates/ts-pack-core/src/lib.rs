//! # tree-sitter-language-pack
//!
//! Pre-compiled tree-sitter grammars for 371 programming languages with
//! a unified API for parsing, analysis, and intelligent code chunking.
//!
//! ## Quick Start
//!
//! ```no_run
//! use tree_sitter_language_pack::{ProcessConfig, available_languages, has_language, process};
//!
//! // Check available languages
//! let langs = available_languages();
//! assert!(has_language("python"));
//!
//! // Process source code
//! let config = ProcessConfig::new("python").all();
//! let result = process("def hello(): pass", &config).unwrap();
//! println!("Language: {}", result.language);
//! println!("Functions: {}", result.structure.len());
//! ```
//!
//! ## Parser API
//!
//! ```no_run
//! use tree_sitter_language_pack::get_parser;
//!
//! let mut parser = get_parser("python")?;
//! let tree = parser.parse("def foo(): pass").expect("parse failed");
//! assert_eq!(tree.root_node().kind(), "module");
//! # Ok::<(), tree_sitter_language_pack::Error>(())
//! ```
//!
//! ## Modules
//!
//! - [`registry`] - Thread-safe language registry for parser lookup
//! - [`parsing`] - Ownership-safe tree-sitter parser, tree, node, and cursor wrappers
//! - [`process_config`] - Configuration for the [`process`] pipeline
//! - [`pack_config`] - Configuration for the language pack (cache dir, languages to download)
//! - [`error`] - Error types
//!
//! Source-code intelligence and the syntax-aware text splitter are exposed via re-exports
//! from the crate root (for example [`process`] and [`StructureItem`]); their backing
//! modules are internal implementation details.

/// Error types for tree-sitter language pack operations.
pub mod error;
pub(crate) mod extensions;
pub(crate) mod intel;
#[cfg(feature = "serde")]
#[doc(hidden)]
pub mod json_utils;
/// Configuration for the language pack (cache directory, languages to pre-download).
pub mod pack_config;
pub(crate) mod parse;
/// Ownership-safe tree-sitter parser, tree, node, and cursor wrappers for FFI.
pub mod parsing;
/// Configuration for the `process()` pipeline (which analysis features to enable).
pub mod process_config;
pub(crate) mod queries;
/// Process-wide cache of compiled tree-sitter queries (`Arc<Query>`), keyed by language + kind.
pub mod query_cache;
/// Thread-safe language registry mapping names to compiled tree-sitter parsers.
pub mod registry;
pub(crate) mod text_splitter;

#[cfg(feature = "config")]
pub(crate) mod config;
#[cfg(feature = "config")]
pub(crate) mod definitions;
/// Download manager for fetching pre-built parser shared libraries from GitHub releases.
#[cfg(feature = "download")]
pub mod download;

pub use error::Error;
pub use extensions::{detect_language_from_content, detect_language_from_extension, detect_language_from_path};
pub use intel::types::{
    ChunkContext, CodeChunk, CommentInfo, CommentKind, DataAttribute, DataNode, DataNodeKind, Diagnostic,
    DiagnosticSeverity, DocSection, DocstringFormat, DocstringInfo, ExportInfo, ExportKind, FileMetrics, ImportInfo,
    ProcessResult, Span, StructureItem, StructureKind, SymbolInfo, SymbolKind,
};
pub use pack_config::{PackConfig, TlsRootsMode};
pub use parsing::{ByteRange, Node, Parser, Point, Tree, TreeCursor};
pub use process_config::ProcessConfig;
pub use queries::{
    get_folds_query, get_highlights_query, get_indents_query, get_injections_query, get_locals_query, get_tags_query,
};
pub use query_cache::{QueryKind, get_query};
pub use registry::LanguageRegistry;
pub use tree_sitter::Language;

#[cfg(feature = "download")]
pub use download::DownloadManager;

use std::sync::LazyLock;
#[cfg(feature = "download")]
use std::sync::{Mutex, RwLock};

static REGISTRY: LazyLock<LanguageRegistry> = LazyLock::new(LanguageRegistry::new);

#[cfg(feature = "download")]
static CACHE_REGISTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(feature = "download")]
static CUSTOM_CACHE_DIR: LazyLock<RwLock<Option<std::path::PathBuf>>> = LazyLock::new(|| RwLock::new(None));

#[cfg(feature = "download")]
static DOWNLOAD_CACHE_LOCK: Mutex<()> = Mutex::new(());

/// Get a tree-sitter [`Language`] by name using the global registry.
///
/// Resolves language aliases (e.g., `"shell"` maps to `"bash"`).
/// When the `download` feature is enabled (default), automatically downloads
/// the parser from GitHub releases if not found locally.
///
/// # Errors
///
/// Returns [`Error::LanguageNotFound`] if the language is not recognized,
/// or [`Error::Download`] if auto-download fails.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::{get_language, Parser};
///
/// let _lang = get_language("python")?;
/// let mut parser = Parser::new();
/// parser.set_language("python")?;
/// let tree = parser.parse("x = 1").expect("parse failed");
/// assert_eq!(tree.root_node().kind(), "module");
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
#[tracing::instrument(level = "debug", skip_all, fields(language = name))]
pub fn get_language(name: &str) -> Result<Language, Error> {
    #[cfg(feature = "download")]
    {
        // ~keep Lock-free probe first: only take the global download lock on an actual miss.
        if let Ok(lang) = REGISTRY.get_language(name) {
            return Ok(lang);
        }
        let _cache_guard = DOWNLOAD_CACHE_LOCK
            .lock()
            .map_err(|e| Error::LockPoisoned(e.to_string()))?;
        // ~keep Double-check under the lock: another thread may have downloaded it.
        if let Ok(lang) = REGISTRY.get_language(name) {
            return Ok(lang);
        }
        ensure_cache_registered()?;
        let cache_dir = effective_cache_dir()?;
        let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
        let resolved = crate::registry::resolve_alias(name);
        dm.ensure_languages(&[resolved])?;
        if let Ok(lang) = REGISTRY.get_language(resolved) {
            return Ok(lang);
        }
        // ~keep Retry once: concurrent cache cleanup can delete the file between extraction and registry lookup.
        // ~keep `ensure_languages` is idempotent and TOCTOU-safe, so the second pass can restore the cache.
        dm.ensure_languages(&[resolved])?;
        REGISTRY.get_language(resolved)
    }

    #[cfg(not(feature = "download"))]
    {
        if let Ok(lang) = REGISTRY.get_language(name) {
            return Ok(lang);
        }
        Err(Error::LanguageNotFound(name.to_string()))
    }
}

/// Get a [`Parser`] pre-configured for the given language.
///
/// This is a convenience function that calls [`get_language`] and configures
/// a new parser in one step.
///
/// # Errors
///
/// Returns [`Error::LanguageNotFound`] if the language is not recognized, or
/// [`Error::ParserSetup`] if the language cannot be applied to the parser.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::get_parser;
///
/// let mut parser = get_parser("rust")?;
/// let tree = parser.parse("fn main() {}").expect("parse failed");
/// assert!(!tree.root_node().has_error());
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
pub fn get_parser(name: &str) -> Result<Parser, Error> {
    let mut parser = Parser::new();
    parser.set_language(name)?;
    Ok(parser)
}

/// Detect language name from a file path or extension.
///
/// This compatibility alias matches the pre-Alef Python binding API.
pub fn detect_language(path: &str) -> Option<&'static str> {
    detect_language_from_path(path).or_else(|| detect_language_from_extension(path.trim_start_matches('.')))
}

/// List all available language names (sorted, deduplicated, includes aliases).
///
/// Returns names of both statically compiled and dynamically loadable languages,
/// plus any configured aliases.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::available_languages;
///
/// let langs = available_languages();
/// for name in &langs {
///     println!("{}", name);
/// }
/// ```
pub fn available_languages() -> Vec<String> {
    #[cfg(feature = "download")]
    let _ = ensure_cache_registered();
    REGISTRY.available_languages()
}

/// Check whether this language can be parsed right now, without downloading.
///
/// Answers from the same lookup [`get_parser`] performs — statically compiled
/// grammars, already-loaded dynamic grammars, and parser shared libraries
/// present in the library and download-cache directories — but never fetches
/// anything over the network.
///
/// Use it to distinguish "we recognise this name" ([`has_language`], which is
/// also `true` for a grammar that still has to be downloaded) from "parsing this
/// will work offline, now". It is likewise independent of the
/// extension-to-language mapping: [`detect_language_from_extension`] consults
/// the static ext table for all 371 grammars regardless of what is installed.
///
/// The first `true` answer for a dynamic grammar loads its shared library, since
/// loading is the only way to know it is usable; loads are cached process-wide,
/// so repeat calls are cheap. ~keep
///
/// ```no_run
/// use tree_sitter_language_pack::{detect_language_from_extension, has_parser};
///
/// if let Some(lang) = detect_language_from_extension("feature") {
///     if has_parser(lang) {
///         // the gherkin parser is installed — parse without touching the network
///     } else {
///         // we know it's gherkin, but the parser still has to be downloaded
///     }
/// }
/// ```
pub fn has_parser(name: &str) -> bool {
    // ~keep Register the download cache first, or installed-but-unregistered parsers read as absent.
    #[cfg(feature = "download")]
    let _ = ensure_cache_registered();
    REGISTRY.has_parser(name)
}

/// Check if a language is available by name or alias.
///
/// Returns `true` if the language can be loaded (statically compiled,
/// dynamically available, or a known alias for one of these).
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::has_language;
///
/// assert!(has_language("python"));
/// assert!(has_language("shell")); // alias for "bash"
/// assert!(!has_language("nonexistent_language"));
/// ```
pub fn has_language(name: &str) -> bool {
    #[cfg(feature = "download")]
    let _ = ensure_cache_registered();
    REGISTRY.has_language(name)
}

/// Return the number of available languages.
///
/// Includes statically compiled languages, dynamically loadable languages,
/// and aliases.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::language_count;
///
/// let count = language_count();
/// println!("{} languages available", count);
/// ```
pub fn language_count() -> usize {
    #[cfg(feature = "download")]
    let _ = ensure_cache_registered();
    REGISTRY.language_count()
}

/// Process source code and extract file intelligence using the global registry.
///
/// Parses the source with tree-sitter and extracts metrics, structure, imports,
/// exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
/// the flags set in [`ProcessConfig`].
///
/// # Errors
///
/// Returns [`Error::InvalidRange`] if the config carries a zero-valued limit or
/// the source exceeds [`ProcessConfig::max_source_bytes`],
/// [`Error::ParseTimeout`] if the parse exceeds
/// [`ProcessConfig::parse_timeout_ms`], [`Error::LanguageNotFound`] if the
/// language is unknown, or [`Error::ParseFailed`] if parsing yields no tree.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::{ProcessConfig, process};
///
/// let config = ProcessConfig::new("python").all();
/// let result = process("def hello(): pass", &config).unwrap();
/// println!("Language: {}", result.language);
/// println!("Lines: {}", result.metrics.total_lines);
/// println!("Structures: {}", result.structure.len());
/// ```
#[tracing::instrument(
    level = "debug",
    skip_all,
    fields(language = %config.language, source_bytes = source.len())
)]
pub fn process(source: &str, config: &ProcessConfig) -> Result<ProcessResult, Error> {
    // ~keep Validate before the download below, so a bad config fails without any network I/O.
    config.validate()?;
    config.check_source_size(source.len())?;

    // ~keep Trigger auto-download here; `REGISTRY.process()` does not perform the download fallback.
    #[cfg(feature = "download")]
    get_language(&config.language)?;

    REGISTRY.process(source, config)
}

#[cfg(feature = "download")]
fn ensure_cache_registered() -> Result<(), Error> {
    if CACHE_REGISTERED.load(std::sync::atomic::Ordering::Acquire) {
        return Ok(());
    }
    let cache_dir = effective_cache_dir()?;
    // ~keep `add_extra_libs_dir` uses interior mutability; no outer write lock is needed.
    REGISTRY.add_extra_libs_dir(cache_dir);
    CACHE_REGISTERED.store(true, std::sync::atomic::Ordering::Release);
    Ok(())
}

#[cfg(feature = "download")]
fn effective_cache_dir() -> Result<std::path::PathBuf, Error> {
    let custom = CUSTOM_CACHE_DIR
        .read()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    match custom.as_ref() {
        Some(dir) => Ok(dir.clone()),
        None => DownloadManager::default_cache_dir(env!("CARGO_PKG_VERSION")),
    }
}

/// Initialize the language pack with the given configuration.
///
/// Applies any custom cache directory, then downloads all languages and groups
/// specified in the config. This is the recommended entry point when you want
/// to pre-warm the cache before use.
///
/// # Errors
///
/// Returns an error if configuration cannot be applied or if downloads fail.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::{PackConfig, init};
///
/// let config = PackConfig {
///     cache_dir: None,
///     languages: Some(vec!["python".to_string(), "rust".to_string()]),
///     groups: None,
/// };
/// init(&config).unwrap();
/// ```
#[cfg(feature = "download")]
#[tracing::instrument(level = "info", skip_all)]
pub fn init(config: &PackConfig) -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    configure_inner(config)?;
    if let Some(ref languages) = config.languages {
        let refs: Vec<&str> = languages.iter().map(String::as_str).collect();
        download_inner(&refs)?;
    }
    if let Some(ref groups) = config.groups {
        let cache_dir = effective_cache_dir()?;
        let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
        for group in groups {
            dm.ensure_group(group)?;
        }
    }
    ensure_cache_registered()?;
    tracing::info!(
        languages = config.languages.as_ref().map_or(0, Vec::len),
        groups = config.groups.as_ref().map_or(0, Vec::len),
        "language pack initialized"
    );
    Ok(())
}

/// Apply download configuration without downloading anything.
///
/// Use this to set a custom cache directory before the first call to
/// [`get_language`] or any download function. Changing the cache dir
/// after languages have been registered has no effect on already-loaded
/// languages.
///
/// # Errors
///
/// Returns an error if the lock cannot be acquired.
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
/// use tree_sitter_language_pack::{PackConfig, configure};
///
/// let config = PackConfig {
///     cache_dir: Some(PathBuf::from("/tmp/my-parsers")),
///     languages: None,
///     groups: None,
/// };
/// configure(&config).unwrap();
/// ```
#[cfg(feature = "download")]
#[tracing::instrument(level = "debug", skip_all)]
pub fn configure(config: &PackConfig) -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    configure_inner(config)
}

#[cfg(feature = "download")]
fn configure_inner(config: &PackConfig) -> Result<(), Error> {
    if let Some(ref dir) = config.cache_dir {
        let mut custom = CUSTOM_CACHE_DIR
            .write()
            .map_err(|e| Error::LockPoisoned(e.to_string()))?;
        *custom = Some(dir.clone());
        // ~keep Old cache directories remain registered; per-path scanning and dedup make that acceptable.
        CACHE_REGISTERED.store(false, std::sync::atomic::Ordering::Release);
    }
    Ok(())
}

/// Download specific languages to the local cache.
///
/// Returns the number of requested languages available after the call. Already
/// compiled or cached languages are included in the count.
///
/// # Errors
///
/// Returns an error if any language is not available in the manifest or if
/// the download fails.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::download;
///
/// let count = download(&["python", "rust", "typescript"]).unwrap();
/// println!("Ensured {} languages", count);
/// ```
#[cfg(feature = "download")]
#[tracing::instrument(level = "info", skip_all, fields(requested = names.len()))]
pub fn download(names: &[&str]) -> Result<usize, Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    let count = download_inner(names)?;
    tracing::info!(count, "ensured languages");
    Ok(count)
}

#[cfg(feature = "download")]
fn download_inner(names: &[&str]) -> Result<usize, Error> {
    ensure_cache_registered()?;
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    let unavailable: Vec<&str> = names
        .iter()
        .copied()
        .filter(|name| !REGISTRY.has_language(name))
        .collect();
    dm.ensure_languages(&unavailable)?;
    let unique: std::collections::BTreeSet<&str> = names.iter().copied().collect();
    Ok(unique.len())
}

/// Prefetch grammars: download any not already loadable from disk, then load every
/// requested language into the process registry so a subsequent hot loop only parses.
///
/// Unlike [`download()`], this does not trust in-memory availability — it downloads
/// whenever a grammar is not actually loadable from disk (fixing the case where a
/// known-but-not-downloaded grammar is reported present), then resolves and caches
/// every requested language. Call it once, up front, before a parallel workload.
///
/// # Errors
///
/// Returns [`Error::Download`] if a required grammar cannot be fetched, or
/// [`Error::LanguageNotFound`] if a requested name is unknown.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::prefetch;
///
/// prefetch(&["rust", "python", "go"])?;
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
#[cfg(feature = "download")]
#[tracing::instrument(level = "info", skip_all, fields(requested = languages.len()))]
pub fn prefetch(languages: &[&str]) -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    ensure_cache_registered()?;

    let resolved: Vec<&str> = languages.iter().map(|n| crate::registry::resolve_alias(n)).collect();

    // ~keep Probe real on-disk loadability; `has_language` reports known-but-not-downloaded grammars as present.
    let needs_download: Vec<&str> = resolved
        .iter()
        .copied()
        .filter(|name| REGISTRY.get_language(name).is_err())
        .collect();

    if !needs_download.is_empty() {
        let cache_dir = effective_cache_dir()?;
        let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
        dm.ensure_languages(&needs_download)?;
    }

    // ~keep Load every requested grammar into the process registry cache.
    for name in &resolved {
        REGISTRY.get_language(name)?;
    }
    tracing::info!(loaded = resolved.len(), "prefetched languages");
    Ok(())
}

/// Prefetch grammars by loading each into the registry.
///
/// Without the `download` feature there is no network step — every requested
/// language must already be statically compiled or present on disk.
///
/// # Errors
///
/// Returns [`Error::LanguageNotFound`] if a requested language is not available.
#[cfg(not(feature = "download"))]
#[tracing::instrument(level = "info", skip_all, fields(requested = languages.len()))]
pub fn prefetch(languages: &[&str]) -> Result<(), Error> {
    for raw in languages {
        let name = crate::registry::resolve_alias(raw);
        REGISTRY.get_language(name)?;
    }
    Ok(())
}

/// Download all available languages from the remote manifest.
///
/// Downloads the platform bundle and extracts every library it contains.
/// Languages that appear in the manifest but are absent from the bundle
/// (e.g. grammars that failed to compile at release time) are silently
/// skipped — they are not treated as an error.
///
/// Returns the total number of languages now available (statically compiled
/// plus downloaded and cached).
///
/// # Errors
///
/// Returns an error if the manifest cannot be fetched or the bundle download fails.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::download_all;
///
/// let count = download_all().unwrap();
/// println!("{} languages available", count);
/// ```
#[cfg(feature = "download")]
#[tracing::instrument(level = "info", skip_all)]
pub fn download_all() -> Result<usize, Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    ensure_cache_registered()?;
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    dm.download_all_best_effort()?;
    let count = REGISTRY.language_count();
    tracing::info!(count, "downloaded all available languages");
    Ok(count)
}

/// Download every language in a named group.
///
/// Groups are defined by the remote manifest, not by this crate, and let you
/// ensure a curated set of related grammars in one call instead of listing each
/// name to [`download()`]. Already-cached languages are skipped.
///
/// Call [`manifest_groups`] to discover the group names the manifest actually
/// defines. The published manifest currently defines a single group, `"all"`;
/// earlier revisions of this documentation advertised `"web"`, `"data"`, and
/// `"systems"`, which the manifest has never contained, so every call following
/// that example failed. Do not hardcode a group name without checking. ~keep
///
/// Returns the total number of languages now available (statically compiled
/// plus downloaded and cached).
///
/// # Errors
///
/// Returns [`Error::Download`] if the manifest cannot be fetched, if the group
/// is unknown — the message lists the groups the manifest defines — or if any
/// constituent language fails to download.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::{download_group, manifest_groups};
///
/// let group = manifest_groups()?.into_iter().next().expect("manifest defines no groups");
/// let count = download_group(&group)?;
/// println!("{count} languages available");
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
#[cfg(feature = "download")]
#[tracing::instrument(level = "info", skip_all, fields(group = name))]
pub fn download_group(name: &str) -> Result<usize, Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    ensure_cache_registered()?;
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    dm.ensure_group(name)?;
    let count = REGISTRY.language_count();
    tracing::info!(group = name, count, "downloaded language group");
    Ok(count)
}

/// Return all language names available in the remote manifest (371).
///
/// Fetches (and caches) the remote manifest to discover the full list of
/// downloadable languages. Use [`downloaded_languages`] to list what is
/// already cached locally.
///
/// # Errors
///
/// Returns an error if the manifest cannot be fetched.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::manifest_languages;
///
/// let langs = manifest_languages().unwrap();
/// println!("{} languages available for download", langs.len());
/// ```
#[cfg(feature = "download")]
pub fn manifest_languages() -> Result<Vec<String>, Error> {
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    let manifest = dm.fetch_manifest()?;
    let mut langs: Vec<String> = manifest.languages.keys().cloned().collect();
    langs.sort_unstable();
    Ok(langs)
}

/// Return the names of every language group the remote manifest defines, sorted.
///
/// Group names are manifest data, not a compile-time constant of this crate, so
/// this is the only reliable way to learn what [`download_group`] and
/// [`PackConfig::groups`] accept. The published manifest currently defines just
/// `"all"`. ~keep
///
/// # Errors
///
/// Returns [`Error::Download`] if the manifest cannot be fetched.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::manifest_groups;
///
/// for group in manifest_groups()? {
///     println!("{group}");
/// }
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
#[cfg(feature = "download")]
pub fn manifest_groups() -> Result<Vec<String>, Error> {
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    let manifest = dm.fetch_manifest()?;
    let mut groups: Vec<String> = manifest.groups.keys().cloned().collect();
    groups.sort_unstable();
    Ok(groups)
}

/// Return languages that are already downloaded and cached locally.
///
/// Does not perform any network requests. Returns an empty list if the
/// cache directory does not exist or cannot be read.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::downloaded_languages;
///
/// let langs = downloaded_languages();
/// println!("{} languages already cached", langs.len());
/// ```
#[cfg(feature = "download")]
pub fn downloaded_languages() -> Vec<String> {
    let cache_dir = match effective_cache_dir() {
        Ok(dir) => dir,
        Err(_) => return Vec::new(),
    };
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    dm.installed_languages()
}

/// Delete all cached parser shared libraries.
///
/// Resets the cache registration so the next call to [`get_language`] or
/// a download function will re-register the (now empty) cache directory.
///
/// # Errors
///
/// Returns an error if the cache directory cannot be removed.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::clean_cache;
///
/// clean_cache().unwrap();
/// println!("Cache cleared");
/// ```
#[cfg(feature = "download")]
#[tracing::instrument(level = "info", skip_all)]
pub fn clean_cache() -> Result<(), Error> {
    let _cache_guard = DOWNLOAD_CACHE_LOCK
        .lock()
        .map_err(|e| Error::LockPoisoned(e.to_string()))?;
    let cache_dir = effective_cache_dir()?;
    let dm = DownloadManager::with_cache_dir(env!("CARGO_PKG_VERSION"), cache_dir);
    dm.clean_cache()?;
    CACHE_REGISTERED.store(false, std::sync::atomic::Ordering::Release);
    tracing::info!("cleared parser cache");
    Ok(())
}

/// Return the effective cache directory path.
///
/// This is either the custom path set via [`configure`] / [`init`] or the
/// default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.
///
/// # Errors
///
/// Returns an error if the system cache directory cannot be determined.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::cache_dir;
///
/// let dir = cache_dir().unwrap();
/// println!("Cache directory: {dir}");
/// ```
#[cfg(feature = "download")]
pub fn cache_dir() -> Result<String, Error> {
    effective_cache_dir().map(|p| p.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_languages() {
        let langs = available_languages();
        let _ = langs;
    }

    #[test]
    fn test_has_language() {
        let langs = available_languages();
        if !langs.is_empty() {
            assert!(has_language(&langs[0]));
        }
        assert!(!has_language("nonexistent_language_xyz"));
    }

    #[test]
    fn test_get_language_invalid() {
        let result = get_language("nonexistent_language_xyz");
        assert!(result.is_err());
    }

    #[test]
    #[ignore = "loads all 371 dynamic libraries — run with --ignored"]
    fn test_get_language_and_parse() {
        let langs = available_languages();
        for lang_name in &langs {
            let lang = get_language(lang_name.as_str())
                .unwrap_or_else(|e| panic!("Failed to load language '{lang_name}': {e}"));
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&lang)
                .unwrap_or_else(|e| panic!("Failed to set language '{lang_name}': {e}"));
            let tree = parser.parse("x", None);
            assert!(tree.is_some(), "Parser for '{lang_name}' should parse a string");
        }
    }

    #[test]
    fn test_get_parser() {
        let langs = available_languages();
        if let Some(first) = langs.first() {
            let parser = get_parser(first.as_str());
            assert!(parser.is_ok(), "get_parser should succeed for '{first}'");
        }
    }

    #[test]
    fn test_pack_config_default() {
        let config = PackConfig::default();
        assert!(config.cache_dir.is_none());
        assert!(config.languages.is_none());
        assert!(config.groups.is_none());
    }
}
