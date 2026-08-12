use std::path::PathBuf;

/// Which CA root set the downloader's TLS client should trust.
///
/// `Platform` (the default) uses the host OS trust store via
/// `rustls-platform-verifier` — matching the behaviour of `curl`, `pip`, and
/// `git` and honouring locally installed CAs (corp TLS-intercepting proxies,
/// internal mirrors, WSL2 with Windows-managed certs, RHEL/UBI with extra
/// anchors). `WebPki` uses ureq's bundled Mozilla roots only; pick this on
/// hosts whose platform trust store is intentionally narrowed or where the
/// bundled Mozilla roots are required for reproducibility.
///
/// Selected at process start via the `TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS`
/// environment variable (`platform` or `webpki`, case-insensitive). The enum
/// is exposed publicly for callers that construct a
/// [`crate::download::DownloadManager`] directly via
/// [`crate::download::DownloadManager::with_cache_dir_and_tls`].
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TlsRootsMode {
    #[default]
    /// Use the host OS platform trust store (default); honours locally installed CAs.
    Platform,
    /// Use the bundled Mozilla WebPKI root certificates only.
    WebPki,
}

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

    /// Language groups to pre-download.
    ///
    /// Group names come from the remote manifest, so the valid set is not fixed
    /// by this crate; the published manifest currently defines only `"all"`.
    /// Call [`manifest_groups`](crate::manifest_groups) to enumerate them.
    /// An unknown name makes [`init`](crate::init) fail. ~keep
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
    #[cfg_attr(alef, alef(skip))]
    #[cfg(feature = "config")]
    pub fn from_toml_file(path: &std::path::Path) -> Result<Self, crate::error::Error> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::error::Error::Config(format!("Failed to read {}: {e}", path.display())))?;
        toml::from_str(&content)
            .map_err(|e| crate::error::Error::Config(format!("Failed to parse {}: {e}", path.display())))
    }

    /// Discover configuration by searching for `language-pack.toml` in:
    ///
    /// 1. Current directory and up to 10 parent directories
    /// 2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml`
    /// 3. `~/.config/tree-sitter-language-pack/config.toml`
    ///
    /// Returns `Ok(None)` if no candidate file exists at any searched location.
    ///
    /// Search stops at the first *existing* candidate. If that file exists but
    /// fails to read or parse, this returns `Err` naming the offending path
    /// rather than silently treating it as "no config" and falling through to a
    /// different candidate further up the search order — that would make a
    /// broken `language-pack.toml` indistinguishable from an absent one, and
    /// silently discard the user's cache directory and pre-download list. ~keep
    ///
    /// # Errors
    ///
    /// Returns an error if a candidate configuration file exists but cannot be
    /// read or is not valid TOML.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tree_sitter_language_pack::PackConfig;
    ///
    /// match PackConfig::discover() {
    ///     Ok(Some(config)) => println!("Found config with {:?} languages", config.languages),
    ///     Ok(None) => println!("No config file found"),
    ///     Err(e) => eprintln!("language-pack.toml found but invalid: {e}"),
    /// }
    /// ```
    #[cfg_attr(alef, alef(skip))]
    #[cfg(feature = "config")]
    pub fn discover() -> Result<Option<Self>, crate::error::Error> {
        if let Ok(cwd) = std::env::current_dir() {
            let mut dir: &std::path::Path = cwd.as_path();
            for _ in 0..10 {
                let candidate = dir.join("language-pack.toml");
                if candidate.exists() {
                    return Self::from_toml_file(&candidate).map(Some);
                }
                match dir.parent() {
                    Some(parent) => dir = parent,
                    None => break,
                }
            }
        }

        if let Some(config_dir) = dirs::config_dir() {
            let candidate = config_dir.join("tree-sitter-language-pack").join("config.toml");
            if candidate.exists() {
                return Self::from_toml_file(&candidate).map(Some);
            }
        }

        Ok(None)
    }
}

#[cfg(all(test, feature = "config"))]
mod tests {
    // ~keep Test assertions legitimately use unwrap/expect; production code stays
    // ~keep covered by the crate-wide `unwrap_used`/`expect_used` deny in Cargo.toml.
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    // ~keep `discover()` reads the process-wide current directory, which cannot be
    // ~keep mutated safely in a parallel test run (test-independence). Its search-stop
    // ~keep and error-propagation behaviour is exercised through `from_toml_file`
    // ~keep instead, which is exactly what `discover()` delegates to at each candidate.
    #[test]
    fn should_return_err_naming_the_path_when_config_file_is_malformed_toml() {
        let temp_dir = tempfile::Builder::new()
            .prefix("tslp-pack-config-")
            .tempdir()
            .expect("temp dir should be created");
        let path = temp_dir.path().join("language-pack.toml");
        std::fs::write(&path, "this is not [ valid toml").expect("malformed file should be written");

        let error = PackConfig::from_toml_file(&path).expect_err("malformed TOML must not silently succeed");

        let message = error.to_string();
        assert!(
            message.contains(&path.display().to_string()),
            "error must name the offending path; got: {message}"
        );
    }

    #[test]
    fn should_return_err_naming_the_path_when_config_file_is_missing() {
        let temp_dir = tempfile::Builder::new()
            .prefix("tslp-pack-config-")
            .tempdir()
            .expect("temp dir should be created");
        let path = temp_dir.path().join("missing.toml");

        let error = PackConfig::from_toml_file(&path).expect_err("missing file must error, not silently produce None");

        let message = error.to_string();
        assert!(
            message.contains(&path.display().to_string()),
            "error must name the missing path; got: {message}"
        );
    }

    #[test]
    fn should_return_ok_when_config_file_is_well_formed_toml() {
        let temp_dir = tempfile::Builder::new()
            .prefix("tslp-pack-config-")
            .tempdir()
            .expect("temp dir should be created");
        let path = temp_dir.path().join("language-pack.toml");
        std::fs::write(&path, "languages = [\"python\", \"rust\"]\n").expect("config file should be written");

        let config = PackConfig::from_toml_file(&path).expect("well-formed config should parse");

        assert_eq!(config.languages, Some(vec!["python".to_string(), "rust".to_string()]));
    }
}
