use std::path::PathBuf;

/// Configuration for the tree-sitter language pack.
///
/// Controls cache directory and which languages to pre-download.
/// Can be loaded from a TOML file, constructed programmatically,
/// or passed as a dict/object from language bindings.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::PackConfig;
///
/// let config = PackConfig {
///     cache_dir: None,
///     languages: Some(vec!["python".to_string(), "rust".to_string()]),
///     groups: None,
/// };
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(
    any(feature = "config", feature = "download"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PackConfig {
    /// Override default cache directory.
    ///
    /// Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`
    #[cfg_attr(any(feature = "config", feature = "download"), serde(default))]
    pub cache_dir: Option<PathBuf>,

    /// Languages to pre-download on init.
    ///
    /// Each entry is a language name (e.g. `"python"`, `"rust"`).
    #[cfg_attr(any(feature = "config", feature = "download"), serde(default))]
    pub languages: Option<Vec<String>>,

    /// Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`).
    #[cfg_attr(any(feature = "config", feature = "download"), serde(default))]
    pub groups: Option<Vec<String>>,
}

impl PackConfig {
    /// Load configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the TOML is invalid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use tree_sitter_language_pack::PackConfig;
    ///
    /// let config = PackConfig::from_toml_file(Path::new("language-pack.toml")).unwrap();
    /// ```
    #[cfg(feature = "config")]
    pub fn from_toml_file(path: &std::path::Path) -> Result<Self, crate::error::Error> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content)
            .map_err(|e| crate::error::Error::Config(format!("Failed to parse {}: {e}", path.display())))
    }

    /// Discover configuration by searching for `language-pack.toml` in:
    ///
    /// 1. Current directory and up to 10 parent directories
    /// 2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml`
    /// 3. `~/.config/tree-sitter-language-pack/config.toml`
    ///
    /// Returns `None` if no configuration file is found.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tree_sitter_language_pack::PackConfig;
    ///
    /// if let Some(config) = PackConfig::discover() {
    ///     println!("Found config with {:?} languages", config.languages);
    /// }
    /// ```
    #[cfg(feature = "config")]
    pub fn discover() -> Option<Self> {
        // Search CWD upward
        if let Ok(cwd) = std::env::current_dir() {
            let mut dir: &std::path::Path = cwd.as_path();
            for _ in 0..10 {
                let candidate = dir.join("language-pack.toml");
                if candidate.exists() {
                    return Self::from_toml_file(&candidate).ok();
                }
                match dir.parent() {
                    Some(parent) => dir = parent,
                    None => break,
                }
            }
        }

        // XDG/user config
        if let Some(config_dir) = dirs::config_dir() {
            let candidate = config_dir.join("tree-sitter-language-pack").join("config.toml");
            if candidate.exists() {
                return Self::from_toml_file(&candidate).ok();
            }
        }

        None
    }
}
