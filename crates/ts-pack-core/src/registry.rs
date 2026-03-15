use std::collections::HashMap;
#[cfg(feature = "dynamic-loading")]
use std::path::PathBuf;
use tree_sitter::Language;

use crate::error::Error;

// Include the build.rs-generated language table
include!(concat!(env!("OUT_DIR"), "/registry_generated.rs"));

/// Alternative names that resolve to an existing grammar.
const LANGUAGE_ALIASES: &[(&str, &str)] = &[
    ("bazel", "starlark"),
    ("gradle", "groovy"),
    ("ignorefile", "gitignore"),
    ("lisp", "commonlisp"),
    ("makefile", "make"),
    ("shell", "bash"),
];

#[inline(always)]
fn resolve_alias(name: &str) -> &str {
    for &(alias, target) in LANGUAGE_ALIASES {
        if name == alias {
            return target;
        }
    }
    name
}

#[cfg(feature = "dynamic-loading")]
mod dynamic {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::RwLock;
    use tree_sitter::Language;

    use crate::error::Error;

    /// Holds dynamically loaded libraries to keep them alive.
    /// The Library must outlive the Language since Language references code in the loaded library.
    pub(crate) struct DynamicLibs {
        libs: HashMap<String, (libloading::Library, Language)>,
    }

    pub(crate) struct DynamicLoader {
        inner: RwLock<DynamicLibs>,
        pub(crate) libs_dir: PathBuf,
        pub(crate) dynamic_names: Vec<&'static str>,
    }

    impl DynamicLoader {
        pub(crate) fn new(libs_dir: PathBuf, dynamic_names: Vec<&'static str>) -> Self {
            Self {
                inner: RwLock::new(DynamicLibs { libs: HashMap::new() }),
                libs_dir,
                dynamic_names,
            }
        }

        pub(crate) fn get_cached(&self, name: &str) -> Result<Option<Language>, Error> {
            let dynamic = self.inner.read().map_err(|e| Error::LockPoisoned(e.to_string()))?;
            Ok(dynamic.libs.get(name).map(|(_, lang)| lang.clone()))
        }

        pub(crate) fn cached_names(&self) -> Vec<String> {
            if let Ok(dynamic) = self.inner.read() {
                dynamic.libs.keys().cloned().collect()
            } else {
                Vec::new()
            }
        }

        pub(crate) fn lib_file_exists(&self, name: &str) -> bool {
            self.lib_path(name).exists()
        }

        fn lib_path(&self, name: &str) -> PathBuf {
            let lib_name = format!("tree_sitter_{name}");
            let (prefix, ext) = if cfg!(target_os = "macos") {
                ("lib", "dylib")
            } else if cfg!(target_os = "windows") {
                ("", "dll")
            } else {
                ("lib", "so")
            };
            self.libs_dir.join(format!("{prefix}{lib_name}.{ext}"))
        }

        /// Load a language from a specific directory (e.g. download cache).
        /// The loaded library is stored in the shared cache.
        pub(crate) fn load_from_dir(&self, name: &str, dir: &std::path::Path) -> Result<Language, Error> {
            let lib_name = format!("tree_sitter_{name}");
            let (prefix, ext) = if cfg!(target_os = "macos") {
                ("lib", "dylib")
            } else if cfg!(target_os = "windows") {
                ("", "dll")
            } else {
                ("lib", "so")
            };
            let lib_path = dir.join(format!("{prefix}{lib_name}.{ext}"));
            if !lib_path.exists() {
                return Err(Error::LanguageNotFound(format!(
                    "Dynamic library for '{}' not found at {}",
                    name,
                    lib_path.display()
                )));
            }
            self.load_from_path(name, &lib_path)
        }

        pub(crate) fn load(&self, name: &str) -> Result<Language, Error> {
            let lib_path = self.lib_path(name);
            if !lib_path.exists() {
                return Err(Error::LanguageNotFound(format!(
                    "Dynamic library for '{}' not found at {}",
                    name,
                    lib_path.display()
                )));
            }
            self.load_from_path(name, &lib_path)
        }

        fn load_from_path(&self, name: &str, lib_path: &std::path::Path) -> Result<Language, Error> {
            let mut dynamic = self.inner.write().map_err(|e| Error::LockPoisoned(e.to_string()))?;

            // Another thread may have loaded it between our read and write lock
            if let Some((_, lang)) = dynamic.libs.get(name) {
                return Ok(lang.clone());
            }

            let func_name = format!("tree_sitter_{name}");

            // SAFETY: We are loading a known tree-sitter grammar shared library that exports
            // a `tree_sitter_<name>` function returning a pointer to a TSLanguage struct.
            let lib = unsafe { libloading::Library::new(lib_path) }
                .map_err(|e| Error::DynamicLoad(format!("Failed to load library {}: {}", lib_path.display(), e)))?;

            let language = unsafe {
                let func: libloading::Symbol<unsafe extern "C" fn() -> *const tree_sitter::ffi::TSLanguage> =
                    lib.get(func_name.as_bytes()).map_err(|e| {
                        Error::DynamicLoad(format!(
                            "Symbol '{}' not found in {}: {}",
                            func_name,
                            lib_path.display(),
                            e
                        ))
                    })?;
                let ptr = func();
                if ptr.is_null() {
                    return Err(Error::NullLanguagePointer(name.to_string()));
                }
                Language::from_raw(ptr)
            };

            dynamic.libs.insert(name.to_string(), (lib, language.clone()));
            Ok(language)
        }
    }
}

/// Thread-safe registry of tree-sitter language parsers.
///
/// Manages both statically compiled and dynamically loaded language grammars.
/// Use [`LanguageRegistry::new()`] for the default registry, or access the
/// global instance via the module-level convenience functions
/// ([`crate::get_language`], [`crate::available_languages`], etc.).
///
/// # Example
///
/// ```no_run
/// use ts_pack_core::{LanguageRegistry, ProcessConfig};
///
/// let registry = LanguageRegistry::new();
/// let langs = registry.available_languages();
/// println!("Available: {:?}", langs);
///
/// let config = ProcessConfig::new("python").all();
/// let result = registry.process("def hello(): pass", &config).unwrap();
/// println!("Structure: {:?}", result.structure);
/// ```
pub struct LanguageRegistry {
    static_lookup: HashMap<&'static str, fn() -> Language>,
    #[cfg(feature = "dynamic-loading")]
    dynamic_loader: dynamic::DynamicLoader,
    /// Additional library directories to search (e.g. download cache).
    #[cfg(feature = "dynamic-loading")]
    extra_lib_dirs: Vec<PathBuf>,
}

impl LanguageRegistry {
    /// Create a new registry populated with all statically compiled languages.
    ///
    /// When the `dynamic-loading` feature is enabled, the registry also knows
    /// about dynamically loadable grammars and will load them on demand.
    pub fn new() -> Self {
        let mut static_lookup = HashMap::with_capacity(STATIC_LANGUAGES.len());
        for &(name, loader) in STATIC_LANGUAGES {
            static_lookup.insert(name, loader);
        }

        Self {
            static_lookup,
            #[cfg(feature = "dynamic-loading")]
            dynamic_loader: dynamic::DynamicLoader::new(PathBuf::from(LIBS_DIR), DYNAMIC_LANGUAGE_NAMES.to_vec()),
            #[cfg(feature = "dynamic-loading")]
            extra_lib_dirs: Vec::new(),
        }
    }

    /// Create a registry with a custom directory for dynamic libraries.
    ///
    /// Overrides the default build-time library directory. Useful when
    /// dynamic grammar shared libraries are stored in a non-standard location.
    #[cfg(feature = "dynamic-loading")]
    pub fn with_libs_dir(libs_dir: PathBuf) -> Self {
        let mut reg = Self::new();
        reg.dynamic_loader.libs_dir = libs_dir;
        reg
    }

    /// Add an additional directory to search for dynamic libraries.
    ///
    /// When [`get_language`](Self::get_language) cannot find a grammar in the
    /// primary library directory, it searches these extra directories in order.
    /// Typically used by the download system to register its cache directory.
    #[cfg(feature = "dynamic-loading")]
    pub fn add_extra_libs_dir(&mut self, dir: PathBuf) {
        if !self.extra_lib_dirs.contains(&dir) {
            self.extra_lib_dirs.push(dir);
        }
    }

    /// Get a tree-sitter [`Language`] by name.
    ///
    /// Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
    /// then looks up the language in the static table. When the `dynamic-loading`
    /// feature is enabled, falls back to loading a shared library on demand.
    ///
    /// # Errors
    ///
    /// Returns [`Error::LanguageNotFound`] if the name (after alias resolution)
    /// does not match any known grammar.
    pub fn get_language(&self, name: &str) -> Result<Language, Error> {
        let name = resolve_alias(name);
        // Try static first
        if let Some(loader) = self.static_lookup.get(name) {
            return Ok(loader());
        }

        #[cfg(feature = "dynamic-loading")]
        {
            // Try already-loaded dynamic (read lock)
            if let Some(lang) = self.dynamic_loader.get_cached(name)? {
                return Ok(lang);
            }

            // Try loading from build-time libs dir
            if self.dynamic_loader.dynamic_names.contains(&name) || self.dynamic_loader.lib_file_exists(name) {
                return self.dynamic_loader.load(name);
            }

            // Try loading from extra dirs (e.g. download cache)
            for extra_dir in &self.extra_lib_dirs {
                if self.dynamic_loader.load_from_dir(name, extra_dir).is_ok() {
                    // Re-fetch from cache — load_from_dir inserted it
                    if let Some(lang) = self.dynamic_loader.get_cached(name)? {
                        return Ok(lang);
                    }
                }
            }
        }

        Err(Error::LanguageNotFound(name.to_string()))
    }

    /// List all available language names, sorted and deduplicated.
    ///
    /// Includes statically compiled languages, dynamically loadable languages
    /// (if the `dynamic-loading` feature is enabled), and all configured aliases.
    pub fn available_languages(&self) -> Vec<String> {
        let mut langs: Vec<String> = self.static_lookup.keys().map(|s| s.to_string()).collect();

        #[cfg(feature = "dynamic-loading")]
        {
            langs.extend(self.dynamic_loader.dynamic_names.iter().map(|s| s.to_string()));
            for name in self.dynamic_loader.cached_names() {
                if !langs.contains(&name) {
                    langs.push(name);
                }
            }
        }
        for &(alias, target) in LANGUAGE_ALIASES {
            if langs.iter().any(|lang| lang.as_str() == target) {
                langs.push(alias.to_string());
            }
        }

        langs.sort_unstable();
        langs.dedup();
        langs
    }

    /// Check whether a language is available by name or alias.
    ///
    /// Returns `true` if the language can be loaded, either from the static
    /// table or from a dynamic library on disk.
    pub fn has_language(&self, name: &str) -> bool {
        let name = resolve_alias(name);
        if self.static_lookup.contains_key(name) {
            return true;
        }

        #[cfg(feature = "dynamic-loading")]
        {
            if self.dynamic_loader.dynamic_names.contains(&name) || self.dynamic_loader.lib_file_exists(name) {
                return true;
            }

            for extra_dir in &self.extra_lib_dirs {
                let lib_name = format!("tree_sitter_{name}");
                let (prefix, ext) = if cfg!(target_os = "macos") {
                    ("lib", "dylib")
                } else if cfg!(target_os = "windows") {
                    ("", "dll")
                } else {
                    ("lib", "so")
                };
                if extra_dir.join(format!("{prefix}{lib_name}.{ext}")).exists() {
                    return true;
                }
            }
        }

        false
    }

    /// Return the total number of available languages (including aliases).
    pub fn language_count(&self) -> usize {
        self.available_languages().len()
    }

    /// Parse source code and extract file intelligence based on config in a single pass.
    pub fn process(
        &self,
        source: &str,
        config: &crate::process_config::ProcessConfig,
    ) -> Result<crate::intel::types::ProcessResult, Error> {
        let mut resolved_config = config.clone();
        resolved_config.language = resolve_alias(&config.language).to_string();
        crate::intel::process(source, &resolved_config, self)
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_config::ProcessConfig;

    fn first_available_lang() -> Option<String> {
        let langs = crate::available_languages();
        langs.into_iter().next()
    }

    #[test]
    fn test_registry_process() {
        let Some(lang) = first_available_lang() else { return };
        let registry = LanguageRegistry::new();
        let config = ProcessConfig::new(&lang);
        let result = registry.process("x", &config);
        assert!(result.is_ok(), "registry.process() should succeed");
        let intel = result.unwrap();
        assert_eq!(intel.language, lang);
        assert!(intel.metrics.total_lines >= 1);
    }

    #[test]
    fn test_registry_process_with_chunking() {
        let Some(lang) = first_available_lang() else { return };
        let registry = LanguageRegistry::new();
        let config = ProcessConfig::new(&lang).with_chunking(1000);
        let result = registry.process("x", &config);
        assert!(result.is_ok(), "registry.process() with chunking should succeed");
        let intel = result.unwrap();
        assert_eq!(intel.language, lang);
        assert!(!intel.chunks.is_empty());
    }

    #[test]
    fn test_registry_process_invalid_language() {
        let registry = LanguageRegistry::new();
        let config = ProcessConfig::new("nonexistent_lang_xyz");
        let result = registry.process("x", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_has_language_and_count() {
        let registry = LanguageRegistry::new();
        let langs = registry.available_languages();
        assert_eq!(registry.language_count(), langs.len());
        if let Some(lang) = langs.first() {
            assert!(registry.has_language(lang));
        }
        assert!(!registry.has_language("nonexistent_lang_xyz"));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_process_result_serde_roundtrip() {
        let Some(lang) = first_available_lang() else { return };
        let registry = LanguageRegistry::new();
        let source = "x";
        let config = ProcessConfig::new(&lang);
        let intel = registry.process(source, &config).unwrap();
        let json = serde_json::to_string(&intel).expect("serialize should succeed");
        let deserialized: crate::intel::types::ProcessResult =
            serde_json::from_str(&json).expect("deserialize should succeed");
        assert_eq!(deserialized.language, intel.language);
        assert_eq!(deserialized.metrics.total_lines, intel.metrics.total_lines);
        assert_eq!(deserialized.metrics.total_bytes, intel.metrics.total_bytes);
    }
}
