use thiserror::Error;

/// Errors that can occur when using the tree-sitter language pack.
///
/// Covers language lookup failures, parse errors, query errors, and I/O issues.
/// Feature-gated variants are included when `config`, `download`, or related
/// features are enabled.
///
/// # Matching on `Error`
///
/// The set of variants is not stable: new failure modes are added in minor
/// releases, and `Io`, `Json`, and `Toml` exist only under certain feature
/// combinations, so the variant set a downstream crate sees depends on which
/// features it enables. Downstream `match`es must therefore carry a `_` arm;
/// `#[non_exhaustive]` makes the compiler enforce that instead of letting a
/// feature change silently break a build. ~keep
///
/// # C ABI error codes
///
/// Each variant alef can see carries an explicit `alef(error_code = N)`
/// allocation that becomes a member of the generated `AlefFfiErrorCode` C enum.
/// These numbers are a public ABI contract: **an allocated number is never
/// reused after its variant is removed**, and a variant's number never changes,
/// because C callers compare against the value, not the name. New variants take
/// the next free number. 0-4 are reserved by alef, so allocation starts at 100.
/// An unannotated variant is emitted as the unknown code rather than as itself,
/// which silently flattens the taxonomy — annotate every new variant. ~keep
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The requested language name (or alias) was not found in the registry.
    #[error("Language '{0}' not found")]
    #[cfg_attr(alef, alef(error_code = 100))]
    LanguageNotFound(String),

    /// A dynamic shared library could not be loaded at runtime.
    #[error("Dynamic library load error: {0}")]
    #[cfg_attr(alef, alef(error_code = 101))]
    DynamicLoad(String),

    /// The tree-sitter language function returned a null pointer for the given language name.
    #[error("Language function returned null pointer for '{0}'")]
    #[cfg_attr(alef, alef(error_code = 102))]
    NullLanguagePointer(String),

    /// The language could not be applied to the parser (e.g., ABI version mismatch).
    #[error("Failed to set parser language: {0}")]
    #[cfg_attr(alef, alef(error_code = 103))]
    ParserSetup(String),

    /// An internal `RwLock` or `Mutex` was poisoned by a previous panic.
    #[error("Registry lock poisoned: {0}")]
    #[cfg_attr(alef, alef(error_code = 104))]
    LockPoisoned(String),

    /// A configuration file or value was invalid or could not be applied.
    #[error("Configuration error: {0}")]
    #[cfg_attr(alef, alef(error_code = 105))]
    Config(String),

    /// The tree-sitter parser returned no tree for the given source input.
    #[error("Parse failed: parsing returned no tree")]
    #[cfg_attr(alef, alef(error_code = 106))]
    ParseFailed,

    /// The parse was cancelled because it exceeded its configured wall-clock budget.
    ///
    /// Raised only when a budget is configured — see
    /// [`ProcessConfig::parse_timeout_ms`](crate::ProcessConfig::parse_timeout_ms),
    /// which defaults to `None`.
    #[error("Parse cancelled: exceeded the configured budget of {timeout_ms} ms")]
    #[cfg_attr(alef, alef(error_code = 107))]
    ParseTimeout {
        /// The configured wall-clock budget, in milliseconds.
        timeout_ms: u64,
    },

    /// A tree-sitter query could not be compiled or executed.
    #[error("Query error: {0}")]
    #[cfg_attr(alef, alef(error_code = 108))]
    QueryError(String),

    /// A byte range was invalid (e.g., end before start, or out of bounds).
    #[error("Invalid byte range: {0}")]
    #[cfg_attr(alef, alef(error_code = 109))]
    InvalidRange(String),

    /// A filesystem or network I/O operation failed.
    #[cfg(not(alef))]
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A JSON value could not be parsed (requires `config` or `download` feature).
    #[cfg(all(not(alef), any(feature = "config", feature = "download")))]
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    /// A TOML configuration file could not be parsed (requires `config` feature).
    #[cfg(all(not(alef), feature = "config"))]
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    /// A parser download from GitHub releases failed.
    #[error("Download error: {0}")]
    #[cfg_attr(alef, alef(error_code = 110))]
    Download(String),

    /// The downloaded file's SHA-256 digest did not match the manifest's expected value.
    #[error("Checksum mismatch for '{file}': expected {expected}, got {actual}")]
    #[cfg_attr(alef, alef(error_code = 111))]
    ChecksumMismatch {
        /// Path of the file whose checksum was verified.
        file: String,
        /// Expected SHA-256 hex digest from the manifest.
        expected: String,
        /// Actual SHA-256 hex digest computed from the downloaded bytes.
        actual: String,
    },

    /// The cross-process download cache lock file could not be acquired or created.
    #[error("Download cache lock error: {0}")]
    #[cfg_attr(alef, alef(error_code = 112))]
    CacheLock(String),
}
