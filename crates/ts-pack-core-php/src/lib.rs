#![allow(dead_code, unused_imports, unused_variables)]
#![allow(
    clippy::too_many_arguments,
    clippy::let_unit_value,
    clippy::needless_borrow,
    clippy::map_identity,
    clippy::just_underscores_and_digits
)]

use ext_php_rs::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\ExtractionPattern")]
pub struct ExtractionPattern {
    /// The tree-sitter query string (S-expression).
    #[php(prop, name = "query")]
    pub query: String,
    /// What to include in each capture result.
    #[php(prop, name = "capture_output")]
    pub capture_output: String,
    /// Field names to extract from child nodes of each capture.
    /// Maps a label to a tree-sitter field name used with `child_by_field_name`.
    #[php(prop, name = "child_fields")]
    pub child_fields: Vec<String>,
    /// Maximum number of matches to return. `None` means unlimited.
    #[php(prop, name = "max_results")]
    pub max_results: Option<i64>,
    /// Restrict matches to a byte range `(start, end)`.
    #[php(prop, name = "byte_range")]
    pub byte_range: Option<String>,
}

#[php_impl]
impl ExtractionPattern {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\ExtractionConfig")]
pub struct ExtractionConfig {
    /// The language name (e.g., `"python"`).
    #[php(prop, name = "language")]
    pub language: String,
    /// Named patterns to run. Keys become the keys in `ExtractionResult::results`.
    #[php(prop, name = "patterns")]
    pub patterns: String,
}

#[php_impl]
impl ExtractionConfig {
    pub fn __construct(language: Option<String>, patterns: Option<String>) -> Self {
        Self {
            language: language.unwrap_or_default(),
            patterns: patterns.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\CaptureResult")]
#[allow(clippy::similar_names)]
pub struct CaptureResult {
    /// The capture name from the query (e.g., `"fn_name"`).
    #[php(prop, name = "name")]
    pub name: String,
    /// The `NodeInfo` snapshot, present when `CaptureOutput` is `Node` or `Full`.
    pub node: Option<NodeInfo>,
    /// The matched source text, present when `CaptureOutput` is `Text` or `Full`.
    #[php(prop, name = "text")]
    pub text: Option<String>,
    /// Values of requested child fields, keyed by field name.
    #[php(prop, name = "child_fields")]
    pub child_fields: String,
    /// Byte offset where this capture starts in the source.
    #[php(prop, name = "start_byte")]
    pub start_byte: i64,
}

#[php_impl]
impl CaptureResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_node(&self) -> Option<NodeInfo> {
        self.node.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\MatchResult")]
pub struct MatchResult {
    /// The pattern index within the query that produced this match.
    #[php(prop, name = "pattern_index")]
    pub pattern_index: i64,
    /// The captures for this match.
    pub captures: Vec<CaptureResult>,
}

#[php_impl]
impl MatchResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_captures(&self) -> Vec<CaptureResult> {
        self.captures.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\PatternResult")]
pub struct PatternResult {
    /// The individual matches.
    pub matches: Vec<MatchResult>,
    /// Total number of matches before `max_results` truncation.
    #[php(prop, name = "total_count")]
    pub total_count: i64,
}

#[php_impl]
impl PatternResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_matches(&self) -> Vec<MatchResult> {
        self.matches.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\ExtractionResult")]
pub struct ExtractionResult {
    /// The language that was used.
    #[php(prop, name = "language")]
    pub language: String,
    /// Results keyed by pattern name.
    #[php(prop, name = "results")]
    pub results: String,
}

#[php_impl]
impl ExtractionResult {
    pub fn __construct(language: Option<String>, results: Option<String>) -> Self {
        Self {
            language: language.unwrap_or_default(),
            results: results.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\PatternValidation")]
pub struct PatternValidation {
    /// Whether the pattern compiled successfully.
    #[php(prop, name = "valid")]
    pub valid: bool,
    /// Names of captures defined in the query.
    #[php(prop, name = "capture_names")]
    pub capture_names: Vec<String>,
    /// Number of patterns in the query.
    #[php(prop, name = "pattern_count")]
    pub pattern_count: i64,
    /// Non-fatal warnings (e.g., unused captures).
    #[php(prop, name = "warnings")]
    pub warnings: Vec<String>,
    /// Fatal errors (e.g., query syntax errors).
    #[php(prop, name = "errors")]
    pub errors: Vec<String>,
}

#[php_impl]
impl PatternValidation {
    pub fn __construct(
        valid: Option<bool>,
        capture_names: Option<Vec<String>>,
        pattern_count: Option<i64>,
        warnings: Option<Vec<String>>,
        errors: Option<Vec<String>>,
    ) -> Self {
        Self {
            valid: valid.unwrap_or_default(),
            capture_names: capture_names.unwrap_or_default(),
            pattern_count: pattern_count.unwrap_or_default(),
            warnings: warnings.unwrap_or_default(),
            errors: errors.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\ValidationResult")]
pub struct ValidationResult {
    /// Whether all patterns are valid.
    #[php(prop, name = "valid")]
    pub valid: bool,
    /// Per-pattern validation details.
    #[php(prop, name = "patterns")]
    pub patterns: String,
}

#[php_impl]
impl ValidationResult {
    pub fn __construct(valid: Option<bool>, patterns: Option<String>) -> Self {
        Self {
            valid: valid.unwrap_or_default(),
            patterns: patterns.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\Span")]
pub struct Span {
    #[php(prop, name = "start_byte")]
    pub start_byte: i64,
    #[php(prop, name = "end_byte")]
    pub end_byte: i64,
    #[php(prop, name = "start_line")]
    pub start_line: i64,
    #[php(prop, name = "start_column")]
    pub start_column: i64,
    #[php(prop, name = "end_line")]
    pub end_line: i64,
    #[php(prop, name = "end_column")]
    pub end_column: i64,
}

#[php_impl]
impl Span {
    pub fn __construct(
        start_byte: Option<i64>,
        end_byte: Option<i64>,
        start_line: Option<i64>,
        start_column: Option<i64>,
        end_line: Option<i64>,
        end_column: Option<i64>,
    ) -> Self {
        Self {
            start_byte: start_byte.unwrap_or_default(),
            end_byte: end_byte.unwrap_or_default(),
            start_line: start_line.unwrap_or_default(),
            start_column: start_column.unwrap_or_default(),
            end_line: end_line.unwrap_or_default(),
            end_column: end_column.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\ProcessResult")]
#[allow(clippy::similar_names)]
pub struct ProcessResult {
    #[php(prop, name = "language")]
    pub language: String,
    pub metrics: FileMetrics,
    pub structure: Vec<StructureItem>,
    pub imports: Vec<ImportInfo>,
    pub exports: Vec<ExportInfo>,
    pub comments: Vec<CommentInfo>,
    pub docstrings: Vec<DocstringInfo>,
    pub symbols: Vec<SymbolInfo>,
    pub diagnostics: Vec<Diagnostic>,
    pub chunks: Vec<CodeChunk>,
    /// Results of custom extraction patterns (when `config.extractions` is set).
    #[php(prop, name = "extractions")]
    pub extractions: String,
}

#[php_impl]
impl ProcessResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_metrics(&self) -> FileMetrics {
        self.metrics.clone()
    }

    #[php(getter)]
    pub fn get_structure(&self) -> Vec<StructureItem> {
        self.structure.clone()
    }

    #[php(getter)]
    pub fn get_imports(&self) -> Vec<ImportInfo> {
        self.imports.clone()
    }

    #[php(getter)]
    pub fn get_exports(&self) -> Vec<ExportInfo> {
        self.exports.clone()
    }

    #[php(getter)]
    pub fn get_comments(&self) -> Vec<CommentInfo> {
        self.comments.clone()
    }

    #[php(getter)]
    pub fn get_docstrings(&self) -> Vec<DocstringInfo> {
        self.docstrings.clone()
    }

    #[php(getter)]
    pub fn get_symbols(&self) -> Vec<SymbolInfo> {
        self.symbols.clone()
    }

    #[php(getter)]
    pub fn get_diagnostics(&self) -> Vec<Diagnostic> {
        self.diagnostics.clone()
    }

    #[php(getter)]
    pub fn get_chunks(&self) -> Vec<CodeChunk> {
        self.chunks.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\FileMetrics")]
pub struct FileMetrics {
    #[php(prop, name = "total_lines")]
    pub total_lines: i64,
    #[php(prop, name = "code_lines")]
    pub code_lines: i64,
    #[php(prop, name = "comment_lines")]
    pub comment_lines: i64,
    #[php(prop, name = "blank_lines")]
    pub blank_lines: i64,
    #[php(prop, name = "total_bytes")]
    pub total_bytes: i64,
    #[php(prop, name = "node_count")]
    pub node_count: i64,
    #[php(prop, name = "error_count")]
    pub error_count: i64,
    #[php(prop, name = "max_depth")]
    pub max_depth: i64,
}

#[php_impl]
impl FileMetrics {
    pub fn __construct(
        total_lines: Option<i64>,
        code_lines: Option<i64>,
        comment_lines: Option<i64>,
        blank_lines: Option<i64>,
        total_bytes: Option<i64>,
        node_count: Option<i64>,
        error_count: Option<i64>,
        max_depth: Option<i64>,
    ) -> Self {
        Self {
            total_lines: total_lines.unwrap_or_default(),
            code_lines: code_lines.unwrap_or_default(),
            comment_lines: comment_lines.unwrap_or_default(),
            blank_lines: blank_lines.unwrap_or_default(),
            total_bytes: total_bytes.unwrap_or_default(),
            node_count: node_count.unwrap_or_default(),
            error_count: error_count.unwrap_or_default(),
            max_depth: max_depth.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\StructureItem")]
pub struct StructureItem {
    #[php(prop, name = "kind")]
    pub kind: String,
    #[php(prop, name = "name")]
    pub name: Option<String>,
    #[php(prop, name = "visibility")]
    pub visibility: Option<String>,
    pub span: Span,
    pub children: Vec<StructureItem>,
    #[php(prop, name = "decorators")]
    pub decorators: Vec<String>,
    #[php(prop, name = "doc_comment")]
    pub doc_comment: Option<String>,
    #[php(prop, name = "signature")]
    pub signature: Option<String>,
    pub body_span: Option<Span>,
}

#[php_impl]
impl StructureItem {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_span(&self) -> Span {
        self.span.clone()
    }

    #[php(getter)]
    pub fn get_children(&self) -> Vec<StructureItem> {
        self.children.clone()
    }

    #[php(getter)]
    pub fn get_body_span(&self) -> Option<Span> {
        self.body_span.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\CommentInfo")]
pub struct CommentInfo {
    #[php(prop, name = "text")]
    pub text: String,
    #[php(prop, name = "kind")]
    pub kind: String,
    pub span: Span,
    #[php(prop, name = "associated_node")]
    pub associated_node: Option<String>,
}

#[php_impl]
impl CommentInfo {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\DocstringInfo")]
pub struct DocstringInfo {
    #[php(prop, name = "text")]
    pub text: String,
    #[php(prop, name = "format")]
    pub format: String,
    pub span: Span,
    #[php(prop, name = "associated_item")]
    pub associated_item: Option<String>,
    pub parsed_sections: Vec<DocSection>,
}

#[php_impl]
impl DocstringInfo {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_span(&self) -> Span {
        self.span.clone()
    }

    #[php(getter)]
    pub fn get_parsed_sections(&self) -> Vec<DocSection> {
        self.parsed_sections.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\DocSection")]
pub struct DocSection {
    #[php(prop, name = "kind")]
    pub kind: String,
    #[php(prop, name = "name")]
    pub name: Option<String>,
    #[php(prop, name = "description")]
    pub description: String,
}

#[php_impl]
impl DocSection {
    pub fn __construct(kind: Option<String>, name: Option<String>, description: Option<String>) -> Self {
        Self {
            kind: kind.unwrap_or_default(),
            name,
            description: description.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\ImportInfo")]
pub struct ImportInfo {
    #[php(prop, name = "source")]
    pub source: String,
    #[php(prop, name = "items")]
    pub items: Vec<String>,
    #[php(prop, name = "alias")]
    pub alias: Option<String>,
    #[php(prop, name = "is_wildcard")]
    pub is_wildcard: bool,
    pub span: Span,
}

#[php_impl]
impl ImportInfo {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\ExportInfo")]
pub struct ExportInfo {
    #[php(prop, name = "name")]
    pub name: String,
    #[php(prop, name = "kind")]
    pub kind: String,
    pub span: Span,
}

#[php_impl]
impl ExportInfo {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\SymbolInfo")]
pub struct SymbolInfo {
    #[php(prop, name = "name")]
    pub name: String,
    #[php(prop, name = "kind")]
    pub kind: String,
    pub span: Span,
    #[php(prop, name = "type_annotation")]
    pub type_annotation: Option<String>,
    #[php(prop, name = "doc")]
    pub doc: Option<String>,
}

#[php_impl]
impl SymbolInfo {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\Diagnostic")]
pub struct Diagnostic {
    #[php(prop, name = "message")]
    pub message: String,
    #[php(prop, name = "severity")]
    pub severity: String,
    pub span: Span,
}

#[php_impl]
impl Diagnostic {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\CodeChunk")]
pub struct CodeChunk {
    #[php(prop, name = "content")]
    pub content: String,
    #[php(prop, name = "start_byte")]
    pub start_byte: i64,
    #[php(prop, name = "end_byte")]
    pub end_byte: i64,
    #[php(prop, name = "start_line")]
    pub start_line: i64,
    #[php(prop, name = "end_line")]
    pub end_line: i64,
    pub metadata: ChunkContext,
}

#[php_impl]
impl CodeChunk {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_metadata(&self) -> ChunkContext {
        self.metadata.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\ChunkContext")]
pub struct ChunkContext {
    #[php(prop, name = "language")]
    pub language: String,
    #[php(prop, name = "chunk_index")]
    pub chunk_index: i64,
    #[php(prop, name = "total_chunks")]
    pub total_chunks: i64,
    #[php(prop, name = "node_types")]
    pub node_types: Vec<String>,
    #[php(prop, name = "context_path")]
    pub context_path: Vec<String>,
    #[php(prop, name = "symbols_defined")]
    pub symbols_defined: Vec<String>,
    pub comments: Vec<CommentInfo>,
    pub docstrings: Vec<DocstringInfo>,
    #[php(prop, name = "has_error_nodes")]
    pub has_error_nodes: bool,
}

#[php_impl]
impl ChunkContext {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_comments(&self) -> Vec<CommentInfo> {
        self.comments.clone()
    }

    #[php(getter)]
    pub fn get_docstrings(&self) -> Vec<DocstringInfo> {
        self.docstrings.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\NodeInfo")]
#[allow(clippy::similar_names)]
pub struct NodeInfo {
    /// The grammar type name (e.g., "function_definition", "identifier").
    #[php(prop, name = "kind")]
    pub kind: String,
    /// Whether this is a named node (vs anonymous like punctuation).
    #[php(prop, name = "is_named")]
    pub is_named: bool,
    /// Start byte offset in source.
    #[php(prop, name = "start_byte")]
    pub start_byte: i64,
    /// End byte offset in source.
    #[php(prop, name = "end_byte")]
    pub end_byte: i64,
    /// Start row (zero-indexed).
    #[php(prop, name = "start_row")]
    pub start_row: i64,
    /// Start column (zero-indexed).
    #[php(prop, name = "start_col")]
    pub start_col: i64,
    /// End row (zero-indexed).
    #[php(prop, name = "end_row")]
    pub end_row: i64,
    /// End column (zero-indexed).
    #[php(prop, name = "end_col")]
    pub end_col: i64,
    /// Number of named children.
    #[php(prop, name = "named_child_count")]
    pub named_child_count: i64,
    /// Whether this node is an ERROR node.
    #[php(prop, name = "is_error")]
    pub is_error: bool,
    /// Whether this node is a MISSING node.
    #[php(prop, name = "is_missing")]
    pub is_missing: bool,
}

#[php_impl]
impl NodeInfo {
    pub fn __construct(
        kind: Option<String>,
        is_named: Option<bool>,
        start_byte: Option<i64>,
        end_byte: Option<i64>,
        start_row: Option<i64>,
        start_col: Option<i64>,
        end_row: Option<i64>,
        end_col: Option<i64>,
        named_child_count: Option<i64>,
        is_error: Option<bool>,
        is_missing: Option<bool>,
    ) -> Self {
        Self {
            kind: kind.unwrap_or_default(),
            is_named: is_named.unwrap_or_default(),
            start_byte: start_byte.unwrap_or_default(),
            end_byte: end_byte.unwrap_or_default(),
            start_row: start_row.unwrap_or_default(),
            start_col: start_col.unwrap_or_default(),
            end_row: end_row.unwrap_or_default(),
            end_col: end_col.unwrap_or_default(),
            named_child_count: named_child_count.unwrap_or_default(),
            is_error: is_error.unwrap_or_default(),
            is_missing: is_missing.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\PackConfig")]
pub struct PackConfig {
    /// Override default cache directory.
    ///
    /// Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`
    #[php(prop, name = "cache_dir")]
    pub cache_dir: Option<String>,
    /// Languages to pre-download on init.
    ///
    /// Each entry is a language name (e.g. `"python"`, `"rust"`).
    #[php(prop, name = "languages")]
    pub languages: Option<Vec<String>>,
    /// Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`).
    #[php(prop, name = "groups")]
    pub groups: Option<Vec<String>>,
}

#[php_impl]
impl PackConfig {
    pub fn __construct(cache_dir: Option<String>, languages: Option<Vec<String>>, groups: Option<Vec<String>>) -> Self {
        Self {
            cache_dir,
            languages,
            groups,
        }
    }

    pub fn from_toml_file(path: String) -> PhpResult<PackConfig> {
        tree_sitter_language_pack::PackConfig::from_toml_file(std::path::Path::new(&path))
            .map(|val| val.into())
            .map_err(|e| PhpException::default(e.to_string()))
    }

    pub fn discover() -> Option<PackConfig> {
        tree_sitter_language_pack::PackConfig::discover().map(Into::into)
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\ProcessConfig")]
#[allow(clippy::similar_names)]
pub struct ProcessConfig {
    /// Language name (required).
    #[php(prop, name = "language")]
    pub language: String,
    /// Extract structural items (functions, classes, etc.). Default: true.
    #[php(prop, name = "structure")]
    pub structure: bool,
    /// Extract import statements. Default: true.
    #[php(prop, name = "imports")]
    pub imports: bool,
    /// Extract export statements. Default: true.
    #[php(prop, name = "exports")]
    pub exports: bool,
    /// Extract comments. Default: false.
    #[php(prop, name = "comments")]
    pub comments: bool,
    /// Extract docstrings. Default: false.
    #[php(prop, name = "docstrings")]
    pub docstrings: bool,
    /// Extract symbol definitions. Default: false.
    #[php(prop, name = "symbols")]
    pub symbols: bool,
    /// Include parse diagnostics. Default: false.
    #[php(prop, name = "diagnostics")]
    pub diagnostics: bool,
    /// Maximum chunk size in bytes. `None` disables chunking.
    #[php(prop, name = "chunk_max_size")]
    pub chunk_max_size: Option<i64>,
    /// Custom extraction patterns to run against the parsed tree.
    /// Keys become the keys in `ProcessResult::extractions`.
    #[php(prop, name = "extractions")]
    pub extractions: Option<String>,
}

#[php_impl]
impl ProcessConfig {
    pub fn __construct(
        language: Option<String>,
        structure: Option<bool>,
        imports: Option<bool>,
        exports: Option<bool>,
        comments: Option<bool>,
        docstrings: Option<bool>,
        symbols: Option<bool>,
        diagnostics: Option<bool>,
        chunk_max_size: Option<i64>,
        extractions: Option<String>,
    ) -> Self {
        Self {
            language: language.unwrap_or_default(),
            structure: structure.unwrap_or(true),
            imports: imports.unwrap_or(true),
            exports: exports.unwrap_or(true),
            comments: comments.unwrap_or(false),
            docstrings: docstrings.unwrap_or(false),
            symbols: symbols.unwrap_or(false),
            diagnostics: diagnostics.unwrap_or(false),
            chunk_max_size,
            extractions,
        }
    }

    pub fn with_chunking(&self, max_size: i64) -> ProcessConfig {
        let core_self = tree_sitter_language_pack::ProcessConfig {
            language: Default::default(),
            structure: self.structure,
            imports: self.imports,
            exports: self.exports,
            comments: self.comments,
            docstrings: self.docstrings,
            symbols: self.symbols,
            diagnostics: self.diagnostics,
            chunk_max_size: self.chunk_max_size.map(|v| v as usize),
            extractions: Default::default(),
        };
        core_self.with_chunking(max_size as usize).into()
    }

    pub fn all(&self) -> ProcessConfig {
        let core_self = tree_sitter_language_pack::ProcessConfig {
            language: Default::default(),
            structure: self.structure,
            imports: self.imports,
            exports: self.exports,
            comments: self.comments,
            docstrings: self.docstrings,
            symbols: self.symbols,
            diagnostics: self.diagnostics,
            chunk_max_size: self.chunk_max_size.map(|v| v as usize),
            extractions: Default::default(),
        };
        core_self.all().into()
    }

    pub fn minimal(&self) -> ProcessConfig {
        let core_self = tree_sitter_language_pack::ProcessConfig {
            language: Default::default(),
            structure: self.structure,
            imports: self.imports,
            exports: self.exports,
            comments: self.comments,
            docstrings: self.docstrings,
            symbols: self.symbols,
            diagnostics: self.diagnostics,
            chunk_max_size: self.chunk_max_size.map(|v| v as usize),
            extractions: Default::default(),
        };
        core_self.minimal().into()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> ProcessConfig {
        tree_sitter_language_pack::ProcessConfig::default().into()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\QueryMatch")]
pub struct QueryMatch {
    /// The pattern index that matched (position in the query string).
    #[php(prop, name = "pattern_index")]
    pub pattern_index: i64,
    /// Captures: list of (capture_name, node_info) pairs.
    #[php(prop, name = "captures")]
    pub captures: Vec<String>,
}

#[php_impl]
impl QueryMatch {
    pub fn __construct(pattern_index: Option<i64>, captures: Option<Vec<String>>) -> Self {
        Self {
            pattern_index: pattern_index.unwrap_or_default(),
            captures: captures.unwrap_or_default(),
        }
    }
}

#[derive(Clone)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\LanguageRegistry")]
pub struct LanguageRegistry {
    inner: Arc<tree_sitter_language_pack::LanguageRegistry>,
}

#[php_impl]
impl LanguageRegistry {
    pub fn add_extra_libs_dir(&self, dir: String) {
        self.inner.add_extra_libs_dir(std::path::PathBuf::from(dir))
    }

    pub fn get_language(&self, name: String) -> PhpResult<Language> {
        let result = self
            .inner
            .get_language(&name)
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(Language {
            inner: Arc::new(result),
        })
    }

    pub fn available_languages(&self) -> Vec<String> {
        self.inner.available_languages()
    }

    pub fn has_language(&self, name: String) -> bool {
        self.inner.has_language(&name)
    }

    pub fn language_count(&self) -> i64 {
        self.inner.language_count() as i64
    }

    pub fn process(&self, source: String, config: &ProcessConfig) -> PhpResult<ProcessResult> {
        Err(ext_php_rs::exception::PhpException::default(
            "Not implemented: process".to_string(),
        ))
    }

    pub fn with_libs_dir(libs_dir: String) -> LanguageRegistry {
        Self {
            inner: Arc::new(tree_sitter_language_pack::LanguageRegistry::with_libs_dir(
                std::path::PathBuf::from(libs_dir),
            )),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> LanguageRegistry {
        Self {
            inner: Arc::new(tree_sitter_language_pack::LanguageRegistry::default()),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\ParserManifest")]
pub struct ParserManifest {
    #[php(prop, name = "version")]
    pub version: String,
    pub platforms: HashMap<String, PlatformBundle>,
    pub languages: HashMap<String, LanguageInfo>,
    pub groups: HashMap<String, Vec<String>>,
}

#[php_impl]
impl ParserManifest {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_platforms(&self) -> HashMap<String, PlatformBundle> {
        self.platforms.clone()
    }

    #[php(getter)]
    pub fn get_languages(&self) -> HashMap<String, LanguageInfo> {
        self.languages.clone()
    }

    #[php(getter)]
    pub fn get_groups(&self) -> HashMap<String, Vec<String>> {
        self.groups.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\PlatformBundle")]
pub struct PlatformBundle {
    #[php(prop, name = "url")]
    pub url: String,
    #[php(prop, name = "sha256")]
    pub sha256: String,
    #[php(prop, name = "size")]
    pub size: i64,
}

#[php_impl]
impl PlatformBundle {
    pub fn __construct(url: String, sha256: String, size: i64) -> Self {
        Self { url, sha256, size }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\LanguageInfo")]
pub struct LanguageInfo {
    #[php(prop, name = "group")]
    pub group: String,
    #[php(prop, name = "size")]
    pub size: i64,
}

#[php_impl]
impl LanguageInfo {
    pub fn __construct(group: String, size: i64) -> Self {
        Self { group, size }
    }
}

#[derive(Clone)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\DownloadManager")]
pub struct DownloadManager {
    inner: Arc<tree_sitter_language_pack::DownloadManager>,
}

#[php_impl]
impl DownloadManager {
    pub fn cache_dir(&self) -> String {
        self.inner.cache_dir().to_string_lossy().to_string()
    }

    pub fn installed_languages(&self) -> Vec<String> {
        self.inner.installed_languages()
    }

    pub fn ensure_languages(&self, names: Vec<String>) -> PhpResult<()> {
        Err(ext_php_rs::exception::PhpException::default(
            "Not implemented: ensure_languages".to_string(),
        ))
    }

    pub fn ensure_group(&self, group: String) -> PhpResult<()> {
        self.inner
            .ensure_group(&group)
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(())
    }

    pub fn lib_path(&self, name: String) -> String {
        self.inner.lib_path(&name).to_string_lossy().to_string()
    }

    pub fn fetch_manifest(&self) -> PhpResult<ParserManifest> {
        let result = self
            .inner
            .fetch_manifest()
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result.into())
    }

    pub fn clean_cache(&self) -> PhpResult<()> {
        self.inner
            .clean_cache()
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(())
    }

    pub fn new(version: String) -> PhpResult<DownloadManager> {
        tree_sitter_language_pack::DownloadManager::new(&version)
            .map(|val| Self { inner: Arc::new(val) })
            .map_err(|e| PhpException::default(e.to_string()))
    }

    pub fn with_cache_dir(version: String, cache_dir: String) -> DownloadManager {
        Self {
            inner: Arc::new(tree_sitter_language_pack::DownloadManager::with_cache_dir(
                &version,
                std::path::PathBuf::from(cache_dir),
            )),
        }
    }

    pub fn default_cache_dir(version: String) -> PhpResult<String> {
        tree_sitter_language_pack::DownloadManager::default_cache_dir(&version)
            .map(|val| val.to_string_lossy().to_string())
            .map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\Language")]
pub struct Language {
    inner: Arc<tree_sitter_language_pack::Language>,
}

#[php_impl]
impl Language {}

#[derive(Clone)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\Parser")]
pub struct Parser {
    inner: Arc<tree_sitter_language_pack::Parser>,
}

#[php_impl]
impl Parser {}

#[derive(Clone)]
#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\Tree")]
pub struct Tree {
    inner: Arc<tree_sitter_language_pack::Tree>,
}

#[php_impl]
impl Tree {}

// CaptureOutput enum values
pub const CAPTUREOUTPUT_TEXT: &str = "Text";
pub const CAPTUREOUTPUT_NODE: &str = "Node";
pub const CAPTUREOUTPUT_FULL: &str = "Full";

// StructureKind enum values
pub const STRUCTUREKIND_FUNCTION: &str = "Function";
pub const STRUCTUREKIND_METHOD: &str = "Method";
pub const STRUCTUREKIND_CLASS: &str = "Class";
pub const STRUCTUREKIND_STRUCT: &str = "Struct";
pub const STRUCTUREKIND_INTERFACE: &str = "Interface";
pub const STRUCTUREKIND_ENUM: &str = "Enum";
pub const STRUCTUREKIND_MODULE: &str = "Module";
pub const STRUCTUREKIND_TRAIT: &str = "Trait";
pub const STRUCTUREKIND_IMPL: &str = "Impl";
pub const STRUCTUREKIND_NAMESPACE: &str = "Namespace";
pub const STRUCTUREKIND_OTHER: &str = "Other";

// CommentKind enum values
pub const COMMENTKIND_LINE: &str = "Line";
pub const COMMENTKIND_BLOCK: &str = "Block";
pub const COMMENTKIND_DOC: &str = "Doc";

// DocstringFormat enum values
pub const DOCSTRINGFORMAT_PYTHONTRIPLEQUOTE: &str = "PythonTripleQuote";
pub const DOCSTRINGFORMAT_JSDOC: &str = "JSDoc";
pub const DOCSTRINGFORMAT_RUSTDOC: &str = "Rustdoc";
pub const DOCSTRINGFORMAT_GODOC: &str = "GoDoc";
pub const DOCSTRINGFORMAT_JAVADOC: &str = "JavaDoc";
pub const DOCSTRINGFORMAT_OTHER: &str = "Other";

// ExportKind enum values
pub const EXPORTKIND_NAMED: &str = "Named";
pub const EXPORTKIND_DEFAULT: &str = "Default";
pub const EXPORTKIND_REEXPORT: &str = "ReExport";

// SymbolKind enum values
pub const SYMBOLKIND_VARIABLE: &str = "Variable";
pub const SYMBOLKIND_CONSTANT: &str = "Constant";
pub const SYMBOLKIND_FUNCTION: &str = "Function";
pub const SYMBOLKIND_CLASS: &str = "Class";
pub const SYMBOLKIND_TYPE: &str = "Type";
pub const SYMBOLKIND_INTERFACE: &str = "Interface";
pub const SYMBOLKIND_ENUM: &str = "Enum";
pub const SYMBOLKIND_MODULE: &str = "Module";
pub const SYMBOLKIND_OTHER: &str = "Other";

// DiagnosticSeverity enum values
pub const DIAGNOSTICSEVERITY_ERROR: &str = "Error";
pub const DIAGNOSTICSEVERITY_WARNING: &str = "Warning";
pub const DIAGNOSTICSEVERITY_INFO: &str = "Info";

#[php_class]
#[php(name = "Tree\\Sitter\\Language\\Pack\\TreeSitterLanguagePackApi")]
pub struct TreeSitterLanguagePackApi;

#[php_impl]
impl TreeSitterLanguagePackApi {
    pub fn detect_language_from_extension(ext: String) -> Option<String> {
        tree_sitter_language_pack::detect_language_from_extension(&ext).map(Into::into)
    }

    pub fn detect_language_from_path(path: String) -> Option<String> {
        tree_sitter_language_pack::detect_language_from_path(&path).map(Into::into)
    }

    pub fn extension_ambiguity(ext: String) -> Option<String> {
        None
    }

    pub fn detect_language_from_content(content: String) -> Option<String> {
        tree_sitter_language_pack::detect_language_from_content(&content).map(Into::into)
    }

    pub fn validate_extraction(config: &ExtractionConfig) -> PhpResult<ValidationResult> {
        Err(ext_php_rs::exception::PhpException::default(
            "Not implemented: validate_extraction".to_string(),
        ))
    }

    pub fn process(source: String, config: &ProcessConfig, registry: &LanguageRegistry) -> PhpResult<ProcessResult> {
        Err(ext_php_rs::exception::PhpException::default(
            "Not implemented: process".to_string(),
        ))
    }

    pub fn root_node_info(tree: &Tree) -> NodeInfo {
        tree_sitter_language_pack::root_node_info(&tree.inner).into()
    }

    pub fn find_nodes_by_type(tree: &Tree, node_type: String) -> Vec<NodeInfo> {
        tree_sitter_language_pack::find_nodes_by_type(&tree.inner, &node_type)
            .into_iter()
            .map(Into::into)
            .collect()
    }

    pub fn named_children_info(tree: &Tree) -> Vec<NodeInfo> {
        tree_sitter_language_pack::named_children_info(&tree.inner)
            .into_iter()
            .map(Into::into)
            .collect()
    }

    pub fn parse_string(language: String, source: Vec<u8>) -> PhpResult<Tree> {
        let result = tree_sitter_language_pack::parse_string(&language, &source)
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(Tree {
            inner: Arc::new(result),
        })
    }

    pub fn tree_contains_node_type(tree: &Tree, node_type: String) -> bool {
        tree_sitter_language_pack::tree_contains_node_type(&tree.inner, &node_type)
    }

    pub fn tree_has_error_nodes(tree: &Tree) -> bool {
        tree_sitter_language_pack::tree_has_error_nodes(&tree.inner)
    }

    pub fn tree_to_sexp(tree: &Tree) -> String {
        tree_sitter_language_pack::tree_to_sexp(&tree.inner)
    }

    pub fn tree_error_count(tree: &Tree) -> i64 {
        tree_sitter_language_pack::tree_error_count(&tree.inner) as i64
    }

    pub fn get_highlights_query(language: String) -> Option<String> {
        tree_sitter_language_pack::get_highlights_query(&language).map(Into::into)
    }

    pub fn get_injections_query(language: String) -> Option<String> {
        tree_sitter_language_pack::get_injections_query(&language).map(Into::into)
    }

    pub fn get_locals_query(language: String) -> Option<String> {
        tree_sitter_language_pack::get_locals_query(&language).map(Into::into)
    }

    pub fn run_query(
        tree: &Tree,
        language: String,
        query_source: String,
        source: Vec<u8>,
    ) -> PhpResult<Vec<QueryMatch>> {
        let result = tree_sitter_language_pack::run_query(&tree.inner, &language, &query_source, &source)
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result.into_iter().map(Into::into).collect())
    }

    pub fn split_code(source: String, tree: &Tree, max_chunk_size: i64) -> Vec<String> {
        Vec::new()
    }

    pub fn get_language(name: String) -> PhpResult<Language> {
        let result = tree_sitter_language_pack::get_language(&name)
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(Language {
            inner: Arc::new(result),
        })
    }

    pub fn get_parser(name: String) -> PhpResult<Parser> {
        let result = tree_sitter_language_pack::get_parser(&name)
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(Parser {
            inner: Arc::new(result),
        })
    }

    pub fn available_languages() -> Vec<String> {
        tree_sitter_language_pack::available_languages()
    }

    pub fn has_language(name: String) -> bool {
        tree_sitter_language_pack::has_language(&name)
    }

    pub fn language_count() -> i64 {
        tree_sitter_language_pack::language_count() as i64
    }

    pub fn extract_patterns(source: String, config: &ExtractionConfig) -> PhpResult<ExtractionResult> {
        Err(ext_php_rs::exception::PhpException::default(
            "Not implemented: extract_patterns".to_string(),
        ))
    }

    pub fn init(config: &PackConfig) -> PhpResult<()> {
        Err(ext_php_rs::exception::PhpException::default(
            "Not implemented: init".to_string(),
        ))
    }

    pub fn configure(config: &PackConfig) -> PhpResult<()> {
        Err(ext_php_rs::exception::PhpException::default(
            "Not implemented: configure".to_string(),
        ))
    }

    pub fn download(names: Vec<String>) -> PhpResult<i64> {
        Err(ext_php_rs::exception::PhpException::default(
            "Not implemented: download".to_string(),
        ))
    }

    pub fn download_all() -> PhpResult<i64> {
        let result = tree_sitter_language_pack::download_all()
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result as i64)
    }

    pub fn manifest_languages() -> PhpResult<Vec<String>> {
        let result = tree_sitter_language_pack::manifest_languages()
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result)
    }

    pub fn downloaded_languages() -> Vec<String> {
        tree_sitter_language_pack::downloaded_languages()
    }

    pub fn clean_cache() -> PhpResult<()> {
        let result = tree_sitter_language_pack::clean_cache()
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result)
    }

    pub fn cache_dir() -> PhpResult<String> {
        let result = tree_sitter_language_pack::cache_dir()
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result.to_string_lossy().to_string())
    }
}

impl From<tree_sitter_language_pack::ExtractionPattern> for ExtractionPattern {
    fn from(val: tree_sitter_language_pack::ExtractionPattern) -> Self {
        Self {
            query: val.query,
            capture_output: serde_json::to_value(val.capture_output)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            child_fields: val.child_fields,
            max_results: val.max_results.map(|v| v as i64),
            byte_range: val.byte_range.as_ref().map(|v| format!("{:?}", v)),
        }
    }
}

impl From<ExtractionConfig> for tree_sitter_language_pack::ExtractionConfig {
    fn from(val: ExtractionConfig) -> Self {
        Self {
            language: val.language,
            patterns: Default::default(),
        }
    }
}

impl From<tree_sitter_language_pack::ExtractionConfig> for ExtractionConfig {
    fn from(val: tree_sitter_language_pack::ExtractionConfig) -> Self {
        Self {
            language: val.language,
            patterns: format!("{:?}", val.patterns),
        }
    }
}

impl From<tree_sitter_language_pack::CaptureResult> for CaptureResult {
    fn from(val: tree_sitter_language_pack::CaptureResult) -> Self {
        Self {
            name: val.name,
            node: val.node.map(Into::into),
            text: val.text,
            child_fields: format!("{:?}", val.child_fields),
            start_byte: val.start_byte as i64,
        }
    }
}

impl From<tree_sitter_language_pack::MatchResult> for MatchResult {
    fn from(val: tree_sitter_language_pack::MatchResult) -> Self {
        Self {
            pattern_index: val.pattern_index as i64,
            captures: val.captures.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<tree_sitter_language_pack::PatternResult> for PatternResult {
    fn from(val: tree_sitter_language_pack::PatternResult) -> Self {
        Self {
            matches: val.matches.into_iter().map(Into::into).collect(),
            total_count: val.total_count as i64,
        }
    }
}

impl From<ExtractionResult> for tree_sitter_language_pack::ExtractionResult {
    fn from(val: ExtractionResult) -> Self {
        Self {
            language: val.language,
            results: Default::default(),
        }
    }
}

impl From<tree_sitter_language_pack::ExtractionResult> for ExtractionResult {
    fn from(val: tree_sitter_language_pack::ExtractionResult) -> Self {
        Self {
            language: val.language,
            results: format!("{:?}", val.results),
        }
    }
}

impl From<tree_sitter_language_pack::PatternValidation> for PatternValidation {
    fn from(val: tree_sitter_language_pack::PatternValidation) -> Self {
        Self {
            valid: val.valid,
            capture_names: val.capture_names,
            pattern_count: val.pattern_count as i64,
            warnings: val.warnings,
            errors: val.errors,
        }
    }
}

impl From<ValidationResult> for tree_sitter_language_pack::ValidationResult {
    fn from(val: ValidationResult) -> Self {
        Self {
            valid: val.valid,
            patterns: Default::default(),
        }
    }
}

impl From<tree_sitter_language_pack::ValidationResult> for ValidationResult {
    fn from(val: tree_sitter_language_pack::ValidationResult) -> Self {
        Self {
            valid: val.valid,
            patterns: format!("{:?}", val.patterns),
        }
    }
}

impl From<Span> for tree_sitter_language_pack::Span {
    fn from(val: Span) -> Self {
        Self {
            start_byte: val.start_byte as usize,
            end_byte: val.end_byte as usize,
            start_line: val.start_line as usize,
            start_column: val.start_column as usize,
            end_line: val.end_line as usize,
            end_column: val.end_column as usize,
        }
    }
}

impl From<tree_sitter_language_pack::Span> for Span {
    fn from(val: tree_sitter_language_pack::Span) -> Self {
        Self {
            start_byte: val.start_byte as i64,
            end_byte: val.end_byte as i64,
            start_line: val.start_line as i64,
            start_column: val.start_column as i64,
            end_line: val.end_line as i64,
            end_column: val.end_column as i64,
        }
    }
}

impl From<ProcessResult> for tree_sitter_language_pack::ProcessResult {
    fn from(val: ProcessResult) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<tree_sitter_language_pack::ProcessResult> for ProcessResult {
    fn from(val: tree_sitter_language_pack::ProcessResult) -> Self {
        Self {
            language: val.language,
            metrics: val.metrics.into(),
            structure: val.structure.into_iter().map(Into::into).collect(),
            imports: val.imports.into_iter().map(Into::into).collect(),
            exports: val.exports.into_iter().map(Into::into).collect(),
            comments: val.comments.into_iter().map(Into::into).collect(),
            docstrings: val.docstrings.into_iter().map(Into::into).collect(),
            symbols: val.symbols.into_iter().map(Into::into).collect(),
            diagnostics: val.diagnostics.into_iter().map(Into::into).collect(),
            chunks: val.chunks.into_iter().map(Into::into).collect(),
            extractions: format!("{:?}", val.extractions),
        }
    }
}

impl From<FileMetrics> for tree_sitter_language_pack::FileMetrics {
    fn from(val: FileMetrics) -> Self {
        Self {
            total_lines: val.total_lines as usize,
            code_lines: val.code_lines as usize,
            comment_lines: val.comment_lines as usize,
            blank_lines: val.blank_lines as usize,
            total_bytes: val.total_bytes as usize,
            node_count: val.node_count as usize,
            error_count: val.error_count as usize,
            max_depth: val.max_depth as usize,
        }
    }
}

impl From<tree_sitter_language_pack::FileMetrics> for FileMetrics {
    fn from(val: tree_sitter_language_pack::FileMetrics) -> Self {
        Self {
            total_lines: val.total_lines as i64,
            code_lines: val.code_lines as i64,
            comment_lines: val.comment_lines as i64,
            blank_lines: val.blank_lines as i64,
            total_bytes: val.total_bytes as i64,
            node_count: val.node_count as i64,
            error_count: val.error_count as i64,
            max_depth: val.max_depth as i64,
        }
    }
}

impl From<StructureItem> for tree_sitter_language_pack::StructureItem {
    fn from(val: StructureItem) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<tree_sitter_language_pack::StructureItem> for StructureItem {
    fn from(val: tree_sitter_language_pack::StructureItem) -> Self {
        Self {
            kind: serde_json::to_value(val.kind)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            name: val.name,
            visibility: val.visibility,
            span: val.span.into(),
            children: val.children.into_iter().map(Into::into).collect(),
            decorators: val.decorators,
            doc_comment: val.doc_comment,
            signature: val.signature,
            body_span: val.body_span.map(Into::into),
        }
    }
}

impl From<CommentInfo> for tree_sitter_language_pack::CommentInfo {
    fn from(val: CommentInfo) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<tree_sitter_language_pack::CommentInfo> for CommentInfo {
    fn from(val: tree_sitter_language_pack::CommentInfo) -> Self {
        Self {
            text: val.text,
            kind: serde_json::to_value(val.kind)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            span: val.span.into(),
            associated_node: val.associated_node,
        }
    }
}

impl From<DocstringInfo> for tree_sitter_language_pack::DocstringInfo {
    fn from(val: DocstringInfo) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<tree_sitter_language_pack::DocstringInfo> for DocstringInfo {
    fn from(val: tree_sitter_language_pack::DocstringInfo) -> Self {
        Self {
            text: val.text,
            format: serde_json::to_value(val.format)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            span: val.span.into(),
            associated_item: val.associated_item,
            parsed_sections: val.parsed_sections.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<DocSection> for tree_sitter_language_pack::DocSection {
    fn from(val: DocSection) -> Self {
        Self {
            kind: val.kind,
            name: val.name,
            description: val.description,
        }
    }
}

impl From<tree_sitter_language_pack::DocSection> for DocSection {
    fn from(val: tree_sitter_language_pack::DocSection) -> Self {
        Self {
            kind: val.kind,
            name: val.name,
            description: val.description,
        }
    }
}

impl From<ImportInfo> for tree_sitter_language_pack::ImportInfo {
    fn from(val: ImportInfo) -> Self {
        Self {
            source: val.source,
            items: val.items,
            alias: val.alias,
            is_wildcard: val.is_wildcard,
            span: val.span.into(),
        }
    }
}

impl From<tree_sitter_language_pack::ImportInfo> for ImportInfo {
    fn from(val: tree_sitter_language_pack::ImportInfo) -> Self {
        Self {
            source: val.source,
            items: val.items,
            alias: val.alias,
            is_wildcard: val.is_wildcard,
            span: val.span.into(),
        }
    }
}

impl From<ExportInfo> for tree_sitter_language_pack::ExportInfo {
    fn from(val: ExportInfo) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<tree_sitter_language_pack::ExportInfo> for ExportInfo {
    fn from(val: tree_sitter_language_pack::ExportInfo) -> Self {
        Self {
            name: val.name,
            kind: serde_json::to_value(val.kind)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            span: val.span.into(),
        }
    }
}

impl From<SymbolInfo> for tree_sitter_language_pack::SymbolInfo {
    fn from(val: SymbolInfo) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<tree_sitter_language_pack::SymbolInfo> for SymbolInfo {
    fn from(val: tree_sitter_language_pack::SymbolInfo) -> Self {
        Self {
            name: val.name,
            kind: serde_json::to_value(val.kind)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            span: val.span.into(),
            type_annotation: val.type_annotation,
            doc: val.doc,
        }
    }
}

impl From<Diagnostic> for tree_sitter_language_pack::Diagnostic {
    fn from(val: Diagnostic) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<tree_sitter_language_pack::Diagnostic> for Diagnostic {
    fn from(val: tree_sitter_language_pack::Diagnostic) -> Self {
        Self {
            message: val.message,
            severity: serde_json::to_value(val.severity)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            span: val.span.into(),
        }
    }
}

impl From<CodeChunk> for tree_sitter_language_pack::CodeChunk {
    fn from(val: CodeChunk) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<tree_sitter_language_pack::CodeChunk> for CodeChunk {
    fn from(val: tree_sitter_language_pack::CodeChunk) -> Self {
        Self {
            content: val.content,
            start_byte: val.start_byte as i64,
            end_byte: val.end_byte as i64,
            start_line: val.start_line as i64,
            end_line: val.end_line as i64,
            metadata: val.metadata.into(),
        }
    }
}

impl From<ChunkContext> for tree_sitter_language_pack::ChunkContext {
    fn from(val: ChunkContext) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<tree_sitter_language_pack::ChunkContext> for ChunkContext {
    fn from(val: tree_sitter_language_pack::ChunkContext) -> Self {
        Self {
            language: val.language,
            chunk_index: val.chunk_index as i64,
            total_chunks: val.total_chunks as i64,
            node_types: val.node_types,
            context_path: val.context_path,
            symbols_defined: val.symbols_defined,
            comments: val.comments.into_iter().map(Into::into).collect(),
            docstrings: val.docstrings.into_iter().map(Into::into).collect(),
            has_error_nodes: val.has_error_nodes,
        }
    }
}

impl From<NodeInfo> for tree_sitter_language_pack::NodeInfo {
    fn from(val: NodeInfo) -> Self {
        Self {
            kind: Default::default(),
            is_named: val.is_named,
            start_byte: val.start_byte as usize,
            end_byte: val.end_byte as usize,
            start_row: val.start_row as usize,
            start_col: val.start_col as usize,
            end_row: val.end_row as usize,
            end_col: val.end_col as usize,
            named_child_count: val.named_child_count as usize,
            is_error: val.is_error,
            is_missing: val.is_missing,
        }
    }
}

impl From<tree_sitter_language_pack::NodeInfo> for NodeInfo {
    fn from(val: tree_sitter_language_pack::NodeInfo) -> Self {
        Self {
            kind: format!("{:?}", val.kind),
            is_named: val.is_named,
            start_byte: val.start_byte as i64,
            end_byte: val.end_byte as i64,
            start_row: val.start_row as i64,
            start_col: val.start_col as i64,
            end_row: val.end_row as i64,
            end_col: val.end_col as i64,
            named_child_count: val.named_child_count as i64,
            is_error: val.is_error,
            is_missing: val.is_missing,
        }
    }
}

impl From<PackConfig> for tree_sitter_language_pack::PackConfig {
    fn from(val: PackConfig) -> Self {
        Self {
            cache_dir: val.cache_dir.map(Into::into),
            languages: val.languages,
            groups: val.groups,
        }
    }
}

impl From<tree_sitter_language_pack::PackConfig> for PackConfig {
    fn from(val: tree_sitter_language_pack::PackConfig) -> Self {
        Self {
            cache_dir: val.cache_dir.map(|p| p.to_string_lossy().to_string()),
            languages: val.languages,
            groups: val.groups,
        }
    }
}

impl From<ProcessConfig> for tree_sitter_language_pack::ProcessConfig {
    fn from(val: ProcessConfig) -> Self {
        Self {
            language: Default::default(),
            structure: val.structure,
            imports: val.imports,
            exports: val.exports,
            comments: val.comments,
            docstrings: val.docstrings,
            symbols: val.symbols,
            diagnostics: val.diagnostics,
            chunk_max_size: val.chunk_max_size.map(|v| v as usize),
            extractions: Default::default(),
        }
    }
}

impl From<tree_sitter_language_pack::ProcessConfig> for ProcessConfig {
    fn from(val: tree_sitter_language_pack::ProcessConfig) -> Self {
        Self {
            language: format!("{:?}", val.language),
            structure: val.structure,
            imports: val.imports,
            exports: val.exports,
            comments: val.comments,
            docstrings: val.docstrings,
            symbols: val.symbols,
            diagnostics: val.diagnostics,
            chunk_max_size: val.chunk_max_size.map(|v| v as i64),
            extractions: val.extractions.as_ref().map(|v| format!("{:?}", v)),
        }
    }
}

impl From<QueryMatch> for tree_sitter_language_pack::QueryMatch {
    fn from(val: QueryMatch) -> Self {
        Self {
            pattern_index: val.pattern_index as usize,
            captures: Default::default(),
        }
    }
}

impl From<tree_sitter_language_pack::QueryMatch> for QueryMatch {
    fn from(val: tree_sitter_language_pack::QueryMatch) -> Self {
        Self {
            pattern_index: val.pattern_index as i64,
            captures: val.captures.iter().map(|i| format!("{:?}", i)).collect(),
        }
    }
}

impl From<ParserManifest> for tree_sitter_language_pack::download::ParserManifest {
    fn from(val: ParserManifest) -> Self {
        Self {
            version: val.version,
            platforms: val.platforms.into_iter().map(|(k, v)| (k, v.into())).collect(),
            languages: val.languages.into_iter().map(|(k, v)| (k, v.into())).collect(),
            groups: val.groups.into_iter().collect(),
        }
    }
}

impl From<tree_sitter_language_pack::download::ParserManifest> for ParserManifest {
    fn from(val: tree_sitter_language_pack::download::ParserManifest) -> Self {
        Self {
            version: val.version,
            platforms: val.platforms.into_iter().map(|(k, v)| (k, v.into())).collect(),
            languages: val.languages.into_iter().map(|(k, v)| (k, v.into())).collect(),
            groups: val.groups.into_iter().collect(),
        }
    }
}

impl From<PlatformBundle> for tree_sitter_language_pack::download::PlatformBundle {
    fn from(val: PlatformBundle) -> Self {
        Self {
            url: val.url,
            sha256: val.sha256,
            size: val.size as u64,
        }
    }
}

impl From<tree_sitter_language_pack::download::PlatformBundle> for PlatformBundle {
    fn from(val: tree_sitter_language_pack::download::PlatformBundle) -> Self {
        Self {
            url: val.url,
            sha256: val.sha256,
            size: val.size as i64,
        }
    }
}

impl From<LanguageInfo> for tree_sitter_language_pack::download::LanguageInfo {
    fn from(val: LanguageInfo) -> Self {
        Self {
            group: val.group,
            size: val.size as u64,
        }
    }
}

impl From<tree_sitter_language_pack::download::LanguageInfo> for LanguageInfo {
    fn from(val: tree_sitter_language_pack::download::LanguageInfo) -> Self {
        Self {
            group: val.group,
            size: val.size as i64,
        }
    }
}

/// Convert a `tree_sitter_language_pack::error::Error` error to a PHP exception.
#[allow(dead_code)]
fn error_to_php_err(e: tree_sitter_language_pack::error::Error) -> ext_php_rs::exception::PhpException {
    let msg = e.to_string();
    #[allow(unreachable_patterns)]
    match &e {
        tree_sitter_language_pack::error::Error::LanguageNotFound(..) => {
            ext_php_rs::exception::PhpException::default(format!("[LanguageNotFound] {}", msg))
        }
        tree_sitter_language_pack::error::Error::DynamicLoad(..) => {
            ext_php_rs::exception::PhpException::default(format!("[DynamicLoad] {}", msg))
        }
        tree_sitter_language_pack::error::Error::NullLanguagePointer(..) => {
            ext_php_rs::exception::PhpException::default(format!("[NullLanguagePointer] {}", msg))
        }
        tree_sitter_language_pack::error::Error::ParserSetup(..) => {
            ext_php_rs::exception::PhpException::default(format!("[ParserSetup] {}", msg))
        }
        tree_sitter_language_pack::error::Error::LockPoisoned(..) => {
            ext_php_rs::exception::PhpException::default(format!("[LockPoisoned] {}", msg))
        }
        tree_sitter_language_pack::error::Error::Config(..) => {
            ext_php_rs::exception::PhpException::default(format!("[Config] {}", msg))
        }
        tree_sitter_language_pack::error::Error::ParseFailed => {
            ext_php_rs::exception::PhpException::default(format!("[ParseFailed] {}", msg))
        }
        tree_sitter_language_pack::error::Error::QueryError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[QueryError] {}", msg))
        }
        tree_sitter_language_pack::error::Error::InvalidRange(..) => {
            ext_php_rs::exception::PhpException::default(format!("[InvalidRange] {}", msg))
        }
        tree_sitter_language_pack::error::Error::Io(..) => {
            ext_php_rs::exception::PhpException::default(format!("[Io] {}", msg))
        }
        _ => ext_php_rs::exception::PhpException::default(msg),
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<ExtractionPattern>()
        .class::<ExtractionConfig>()
        .class::<CaptureResult>()
        .class::<MatchResult>()
        .class::<PatternResult>()
        .class::<ExtractionResult>()
        .class::<PatternValidation>()
        .class::<ValidationResult>()
        .class::<Span>()
        .class::<ProcessResult>()
        .class::<FileMetrics>()
        .class::<StructureItem>()
        .class::<CommentInfo>()
        .class::<DocstringInfo>()
        .class::<DocSection>()
        .class::<ImportInfo>()
        .class::<ExportInfo>()
        .class::<SymbolInfo>()
        .class::<Diagnostic>()
        .class::<CodeChunk>()
        .class::<ChunkContext>()
        .class::<NodeInfo>()
        .class::<PackConfig>()
        .class::<ProcessConfig>()
        .class::<QueryMatch>()
        .class::<LanguageRegistry>()
        .class::<ParserManifest>()
        .class::<PlatformBundle>()
        .class::<LanguageInfo>()
        .class::<DownloadManager>()
        .class::<Language>()
        .class::<Parser>()
        .class::<Tree>()
        .class::<TreeSitterLanguagePackApi>()
}
