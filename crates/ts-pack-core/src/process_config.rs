use std::borrow::Cow;

/// One mebibyte, used to express the recommended source-size ceiling. ~keep
const BYTES_PER_MIB: usize = 1024 * 1024;

/// Suggested ceiling for [`ProcessConfig::max_source_bytes`] when parsing untrusted
/// input: 16 MiB.
///
/// This is a *suggestion*, not the default — see [`ProcessConfig::max_source_bytes`],
/// which is `None` (unbounded) so that existing callers are unaffected. ~keep
pub const RECOMMENDED_MAX_SOURCE_BYTES: usize = 16 * BYTES_PER_MIB;

/// Suggested value for [`ProcessConfig::parse_timeout_ms`] when parsing untrusted
/// input: 5000 ms.
///
/// This is a *suggestion*, not the default — see [`ProcessConfig::parse_timeout_ms`],
/// which is `None` (no timeout) so that existing callers are unaffected. ~keep
pub const RECOMMENDED_PARSE_TIMEOUT_MS: u64 = 5_000;

/// Configuration for the `process()` function.
///
/// Controls which analysis features are enabled and whether chunking is performed.
///
/// # Examples
///
/// ```
/// use tree_sitter_language_pack::ProcessConfig;
///
/// // Defaults: structure + imports + exports enabled
/// let config = ProcessConfig::new("python");
///
/// // With chunking
/// let config = ProcessConfig::new("python").with_chunking(1000);
///
/// // Everything enabled
/// let config = ProcessConfig::new("python").all();
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// ~keep `#[non_exhaustive]` belongs here but cannot land yet: it also forbids functional-record
// ~keep update (`Self { .., ..Default::default() }`), which the Alef-generated bindings in
// ~keep ts-pack-core-{py,php,wasm} and packages/{ruby,dart} emit. The generator template must
// ~keep switch to `let mut x = Type::default();` first — see the ts-pack-core-node bindings,
// ~keep which are already written in that style.
pub struct ProcessConfig {
    /// Language name (required).
    pub language: Cow<'static, str>,
    /// Extract structural items (functions, classes, etc.). Default: true.
    #[cfg_attr(feature = "serde", serde(default = "default_true"))]
    pub structure: bool,
    /// Extract import statements. Default: true.
    #[cfg_attr(feature = "serde", serde(default = "default_true"))]
    pub imports: bool,
    /// Extract export statements. Default: true.
    #[cfg_attr(feature = "serde", serde(default = "default_true"))]
    pub exports: bool,
    /// Extract comments. Default: false.
    #[cfg_attr(feature = "serde", serde(default))]
    pub comments: bool,
    /// Extract docstrings. Default: false.
    #[cfg_attr(feature = "serde", serde(default))]
    pub docstrings: bool,
    /// Extract symbol definitions. Default: false.
    #[cfg_attr(feature = "serde", serde(default))]
    pub symbols: bool,
    /// Include parse diagnostics. Default: false.
    #[cfg_attr(feature = "serde", serde(default))]
    pub diagnostics: bool,
    /// Maximum chunk size in bytes. `None` disables chunking.
    ///
    /// `Some(0)` is rejected by [`ProcessConfig::validate`] with
    /// [`Error::InvalidRange`](crate::Error::InvalidRange). A zero-sized chunk
    /// limit previously produced an empty chunk list and silently discarded the
    /// whole source; use `None` to mean "do not chunk". ~keep
    #[cfg_attr(feature = "serde", serde(default))]
    pub chunk_max_size: Option<usize>,
    /// Extract hierarchical key/value data tree from data-format files. Default: false.
    ///
    /// When `true`, [`ProcessResult::data`](crate::ProcessResult::data) is populated
    /// with a [`DataNode`](crate::DataNode) tree for supported languages: JSON, YAML,
    /// TOML, `.properties`, HCL/HOCON, INI, editorconfig, KDL, CUE, CSV, PSV, PO,
    /// nginx config, Caddy config, XML, and DTD.
    ///
    /// For languages outside this set the field is left as `None`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tree_sitter_language_pack::{ProcessConfig, process};
    ///
    /// let config = ProcessConfig::new("json").with_data_extraction(true);
    /// let result = process(r#"{"host": "localhost"}"#, &config).unwrap();
    /// assert!(result.data.is_some());
    /// ```
    #[cfg_attr(feature = "serde", serde(default))]
    pub data_extraction: bool,
    /// Reject source longer than this many bytes instead of parsing it.
    /// Default: `None` (unbounded).
    ///
    /// Tree-sitter allocates and walks proportionally to input size, so an
    /// unbounded parse of attacker-supplied input is a denial-of-service vector.
    /// The default stays unbounded for backward compatibility; services handling
    /// untrusted input should opt in, e.g. with
    /// [`RECOMMENDED_MAX_SOURCE_BYTES`]. ~keep
    ///
    /// Exceeding the limit fails the call with
    /// [`Error::InvalidRange`](crate::Error::InvalidRange) — the source is never
    /// silently truncated.
    #[cfg_attr(feature = "serde", serde(default))]
    pub max_source_bytes: Option<usize>,
    /// Wall-clock budget for the parse step, in milliseconds.
    /// Default: `None` (no timeout).
    ///
    /// Enforced through tree-sitter's parse progress callback, which the parser
    /// invokes periodically; cancellation is therefore granular to that callback
    /// interval rather than exact. A parse that exceeds the budget fails with
    /// [`Error::ParseTimeout`](crate::Error::ParseTimeout). ~keep
    #[cfg_attr(feature = "serde", serde(default))]
    pub parse_timeout_ms: Option<u64>,
}

#[cfg(feature = "serde")]
fn default_true() -> bool {
    true
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            language: Cow::Borrowed(""),
            structure: true,
            imports: true,
            exports: true,
            comments: false,
            docstrings: false,
            symbols: false,
            diagnostics: false,
            chunk_max_size: None,
            data_extraction: false,
            max_source_bytes: None,
            parse_timeout_ms: None,
        }
    }
}

impl ProcessConfig {
    /// Create a new config for the given language with default settings.
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: Cow::Owned(language.into()),
            ..Default::default()
        }
    }

    /// Enable chunking with the given maximum chunk size in bytes.
    pub fn with_chunking(mut self, max_size: usize) -> Self {
        self.chunk_max_size = Some(max_size);
        self
    }

    /// Enable every analysis feature, including data extraction.
    ///
    /// Chunking is not an analysis feature and stays off; enable it with
    /// [`with_chunking`](Self::with_chunking).
    pub fn all(mut self) -> Self {
        self.structure = true;
        self.imports = true;
        self.exports = true;
        self.comments = true;
        self.docstrings = true;
        self.symbols = true;
        self.diagnostics = true;
        self.data_extraction = true;
        self
    }

    /// Disable all analysis features (only metrics computed).
    pub fn minimal(mut self) -> Self {
        self.structure = false;
        self.imports = false;
        self.exports = false;
        self.comments = false;
        self.docstrings = false;
        self.symbols = false;
        self.diagnostics = false;
        self.data_extraction = false;
        self
    }

    /// Enable or disable hierarchical data extraction for data-format files.
    ///
    /// When `true`, [`ProcessResult::data`](crate::ProcessResult::data) is
    /// populated with a key/value tree for supported data-format languages.
    pub fn with_data_extraction(mut self, enabled: bool) -> Self {
        self.data_extraction = enabled;
        self
    }

    /// Reject source longer than `max_bytes` instead of parsing it.
    ///
    /// Pass `None` to restore the default unbounded behaviour. See
    /// [`RECOMMENDED_MAX_SOURCE_BYTES`] for a starting value.
    pub fn with_max_source_bytes(mut self, max_bytes: Option<usize>) -> Self {
        self.max_source_bytes = max_bytes;
        self
    }

    /// Cancel the parse if it exceeds `timeout_ms` milliseconds of wall clock.
    ///
    /// Pass `None` to restore the default (no timeout). See
    /// [`RECOMMENDED_PARSE_TIMEOUT_MS`] for a starting value.
    pub fn with_parse_timeout_ms(mut self, timeout_ms: Option<u64>) -> Self {
        self.parse_timeout_ms = timeout_ms;
        self
    }

    /// Check that the configured limits are usable before they drive a parse.
    ///
    /// Called by [`crate::process`] and
    /// [`LanguageRegistry::process`](crate::LanguageRegistry::process); call it
    /// directly to reject a bad configuration early.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRange`](crate::Error::InvalidRange) when
    /// `chunk_max_size`, `max_source_bytes`, or `parse_timeout_ms` is `Some(0)`.
    /// Zero is always a configuration mistake: `None` is how each of these is
    /// disabled, so `Some(0)` could only mean "produce nothing". ~keep
    pub fn validate(&self) -> Result<(), crate::Error> {
        if self.chunk_max_size == Some(0) {
            return Err(crate::Error::InvalidRange(
                "chunk_max_size must be greater than 0; use None to disable chunking".to_string(),
            ));
        }
        if self.max_source_bytes == Some(0) {
            return Err(crate::Error::InvalidRange(
                "max_source_bytes must be greater than 0; use None for no source-size limit".to_string(),
            ));
        }
        if self.parse_timeout_ms == Some(0) {
            return Err(crate::Error::InvalidRange(
                "parse_timeout_ms must be greater than 0; use None for no parse timeout".to_string(),
            ));
        }
        Ok(())
    }

    /// Check the source against [`max_source_bytes`](Self::max_source_bytes).
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRange`](crate::Error::InvalidRange) when a limit
    /// is configured and `source_bytes` exceeds it.
    pub(crate) fn check_source_size(&self, source_bytes: usize) -> Result<(), crate::Error> {
        let Some(limit) = self.max_source_bytes else {
            return Ok(());
        };
        if source_bytes <= limit {
            return Ok(());
        }
        tracing::warn!(
            language = %self.language,
            source_bytes,
            limit,
            "source exceeds the configured max_source_bytes; refusing to parse"
        );
        Err(crate::Error::InvalidRange(format!(
            "source is {source_bytes} bytes, which exceeds the configured max_source_bytes of {limit}"
        )))
    }
}
