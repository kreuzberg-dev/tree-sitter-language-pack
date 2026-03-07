use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;
use tree_sitter::Language;

use crate::error::Error;

// Include the build.rs-generated language table
include!(concat!(env!("OUT_DIR"), "/registry_generated.rs"));

/// Holds dynamically loaded libraries to keep them alive.
/// The Library must outlive the Language since Language references code in the loaded library.
struct DynamicLibs {
    libs: HashMap<String, (libloading::Library, Language)>,
}

pub struct LanguageRegistry {
    static_lookup: HashMap<&'static str, fn() -> Language>,
    dynamic: RwLock<DynamicLibs>,
    libs_dir: PathBuf,
    dynamic_names: Vec<&'static str>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        let mut static_lookup = HashMap::with_capacity(STATIC_LANGUAGES.len());
        for &(name, loader) in STATIC_LANGUAGES {
            static_lookup.insert(name, loader);
        }

        Self {
            static_lookup,
            dynamic: RwLock::new(DynamicLibs { libs: HashMap::new() }),
            libs_dir: PathBuf::from(LIBS_DIR),
            dynamic_names: DYNAMIC_LANGUAGE_NAMES.to_vec(),
        }
    }

    /// Create a registry with a custom directory for dynamic libraries.
    pub fn with_libs_dir(libs_dir: PathBuf) -> Self {
        let mut reg = Self::new();
        reg.libs_dir = libs_dir;
        reg
    }

    pub fn get_language(&self, name: &str) -> Result<Language, Error> {
        // Try static first
        if let Some(loader) = self.static_lookup.get(name) {
            return Ok(loader());
        }

        // Try already-loaded dynamic (read lock)
        {
            let dynamic = self.dynamic.read().map_err(|e| Error::LockPoisoned(e.to_string()))?;
            if let Some((_, lang)) = dynamic.libs.get(name) {
                return Ok(lang.clone());
            }
        }

        // Try loading dynamically
        if self.dynamic_names.contains(&name) || self.lib_file_exists(name) {
            return self.load_dynamic(name);
        }

        Err(Error::LanguageNotFound(name.to_string()))
    }

    pub fn available_languages(&self) -> Vec<String> {
        let mut langs: Vec<String> = self.static_lookup.keys().map(|s| s.to_string()).collect();
        langs.extend(self.dynamic_names.iter().map(|s| s.to_string()));
        if let Ok(dynamic) = self.dynamic.read() {
            for name in dynamic.libs.keys() {
                if !langs.iter().any(|l| l == name) {
                    langs.push(name.clone());
                }
            }
        }
        langs.sort_unstable();
        langs.dedup();
        langs
    }

    pub fn has_language(&self, name: &str) -> bool {
        self.static_lookup.contains_key(name) || self.dynamic_names.contains(&name) || self.lib_file_exists(name)
    }

    pub fn language_count(&self) -> usize {
        // Avoid allocating the full Vec just for a count
        let mut count = self.static_lookup.len();
        for n in &self.dynamic_names {
            if !self.static_lookup.contains_key(n) {
                count += 1;
            }
        }
        count
    }

    fn lib_file_exists(&self, name: &str) -> bool {
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

    fn load_dynamic(&self, name: &str) -> Result<Language, Error> {
        // Acquire write lock and re-check (handles TOCTOU race from concurrent loads)
        let mut dynamic = self.dynamic.write().map_err(|e| Error::LockPoisoned(e.to_string()))?;

        // Another thread may have loaded it between our read and write lock
        if let Some((_, lang)) = dynamic.libs.get(name) {
            return Ok(lang.clone());
        }

        let lib_path = self.lib_path(name);
        if !lib_path.exists() {
            return Err(Error::LanguageNotFound(format!(
                "Dynamic library for '{}' not found at {}",
                name,
                lib_path.display()
            )));
        }

        let func_name = format!("tree_sitter_{name}");

        // SAFETY: We are loading a known tree-sitter grammar shared library that exports
        // a `tree_sitter_<name>` function returning a pointer to a TSLanguage struct.
        let lib = unsafe { libloading::Library::new(&lib_path) }
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

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}
