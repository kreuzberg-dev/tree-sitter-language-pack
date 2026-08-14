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
    /// Override the BASE directory the parser cache lives under.
    ///
    /// This is a base, not the final library path: the crate appends
    /// `tree-sitter-language-pack/v{version}/libs` to it, exactly as it does to the
    /// platform default. So `cache_dir = "/tmp/my-parsers"` resolves to
    /// `/tmp/my-parsers/tree-sitter-language-pack/v{version}/libs/`.
    ///
    /// The suffix is not cosmetic. It keeps the whole cache tree — manifest, bundles
    /// and lock file included — inside a directory this crate owns and versions.
    /// Earlier releases used this path verbatim, which put those files in the
    /// configured directory's PARENT and let a cache built by one crate version be
    /// reused by another. ~keep
    ///
    /// Default base: the platform cache dir, e.g. `~/.cache` on Linux.
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
    /// Returns `None` if no candidate file exists at any searched location, or
    /// if a candidate exists but cannot be read or parsed. In the latter case a
    /// `tracing::warn!` names the offending path before this returns `None`.
    /// Callers that need to distinguish "no config" from "broken config" — the
    /// CLI does, because it reports a malformed file to the user by path —
    /// should use [`Self::try_discover`] instead. This signature stays
    /// infallible to keep the public API source-compatible. ~keep
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
    #[cfg_attr(alef, alef(skip))]
    #[cfg(feature = "config")]
    pub fn discover() -> Option<Self> {
        match Self::try_discover() {
            Ok(config) => config,
            Err(error) => {
                tracing::warn!(%error, "discovered language-pack.toml could not be loaded; treating as absent");
                None
            }
        }
    }

    /// Discover configuration, reporting a broken candidate file as an error.
    ///
    /// Searches the same locations as [`Self::discover`]. Search stops at the
    /// first *existing* candidate. If that file exists but fails to read or
    /// parse, this returns `Err` naming the offending path rather than
    /// silently treating it as "no config" and falling through to a different
    /// candidate further up the search order — that would make a broken
    /// `language-pack.toml` indistinguishable from an absent one, and silently
    /// discard the user's cache directory and pre-download list. ~keep
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
    /// match PackConfig::try_discover() {
    ///     Ok(Some(config)) => println!("Found config with {:?} languages", config.languages),
    ///     Ok(None) => println!("No config file found"),
    ///     Err(e) => eprintln!("language-pack.toml found but invalid: {e}"),
    /// }
    /// ```
    #[cfg_attr(alef, alef(skip))]
    #[cfg(feature = "config")]
    pub fn try_discover() -> Result<Option<Self>, crate::error::Error> {
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

    // ~keep `discover()`/`try_discover()` read the process-wide current directory,
    // ~keep which cannot be mutated by more than one test at a time in a parallel test
    // ~keep run (test-independence). `CWD_LOCK` serializes the two tests below against
    // ~keep each other, and `CwdOverride` restores the original directory on drop —
    // ~keep even if an assertion panics — so no other test in this binary observes the
    // ~keep temporary directory change.
    static CWD_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    struct CwdOverride {
        original: std::path::PathBuf,
        _guard: std::sync::MutexGuard<'static, ()>,
    }

    impl CwdOverride {
        fn new(dir: &std::path::Path) -> Self {
            let guard = CWD_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let original = std::env::current_dir().expect("current dir should be readable");
            std::env::set_current_dir(dir).expect("current dir should be settable");
            Self {
                original,
                _guard: guard,
            }
        }
    }

    impl Drop for CwdOverride {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.original).expect("original current dir should be restorable");
        }
    }

    #[test]
    fn should_return_err_naming_the_path_when_try_discover_finds_malformed_toml() {
        let temp_dir = tempfile::Builder::new()
            .prefix("tslp-pack-config-discover-")
            .tempdir()
            .expect("temp dir should be created");
        let path = temp_dir.path().join("language-pack.toml");
        std::fs::write(&path, "this is not [ valid toml").expect("malformed file should be written");
        let _cwd = CwdOverride::new(temp_dir.path());

        let error = PackConfig::try_discover().expect_err("malformed TOML must not silently succeed");

        let message = error.to_string();
        assert!(
            message.contains(&path.display().to_string()),
            "error must name the offending path; got: {message}"
        );
    }

    #[test]
    fn should_return_none_when_discover_finds_malformed_toml() {
        let temp_dir = tempfile::Builder::new()
            .prefix("tslp-pack-config-discover-")
            .tempdir()
            .expect("temp dir should be created");
        let path = temp_dir.path().join("language-pack.toml");
        std::fs::write(&path, "this is not [ valid toml").expect("malformed file should be written");
        let _cwd = CwdOverride::new(temp_dir.path());

        let config = PackConfig::discover();

        assert!(
            config.is_none(),
            "a malformed config file must be reported via a warning and treated as absent, \
             not silently accepted; got: {config:?}"
        );
    }

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
