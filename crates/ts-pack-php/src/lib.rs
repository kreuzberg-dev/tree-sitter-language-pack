#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::missing_errors_doc)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::should_implement_trait)]

use ext_php_rs::prelude::*;
use std::sync::Arc;

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct ExtractionPattern {
    pub query: String,
    pub capture_output: String,
    pub child_fields: Vec<String>,
    pub max_results: Option<i64>,
    pub byte_range: Option<String>,
}

#[php_impl]
impl ExtractionPattern {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for ExtractionPattern requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct ExtractionConfig {
    pub language: String,
    pub patterns: String,
}

#[php_impl]
impl ExtractionConfig {
    pub fn __construct(language: String, patterns: String) -> Self {
        Self { language, patterns }
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct CaptureResult {
    pub name: String,
    pub node: Option<NodeInfo>,
    pub text: Option<String>,
    pub child_fields: String,
    pub start_byte: i64,
}

#[php_impl]
impl CaptureResult {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for CaptureResult requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct MatchResult {
    pub pattern_index: i64,
    pub captures: Vec<CaptureResult>,
}

#[php_impl]
impl MatchResult {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for MatchResult requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct PatternResult {
    pub matches: Vec<MatchResult>,
    pub total_count: i64,
}

#[php_impl]
impl PatternResult {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for PatternResult requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct ExtractionResult {
    pub language: String,
    pub results: String,
}

#[php_impl]
impl ExtractionResult {
    pub fn __construct(language: String, results: String) -> Self {
        Self { language, results }
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct PatternValidation {
    pub valid: bool,
    pub capture_names: Vec<String>,
    pub pattern_count: i64,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[php_impl]
impl PatternValidation {
    pub fn __construct(
            valid: bool,
            capture_names: Vec<String>,
            pattern_count: i64,
            warnings: Vec<String>,
            errors: Vec<String>,
        ) -> Self {
        Self { valid, capture_names, pattern_count, warnings, errors }
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct ValidationResult {
    pub valid: bool,
    pub patterns: String,
}

#[php_impl]
impl ValidationResult {
    pub fn __construct(valid: bool, patterns: String) -> Self {
        Self { valid, patterns }
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct Span {
    pub start_byte: i64,
    pub end_byte: i64,
    pub start_line: i64,
    pub start_column: i64,
    pub end_line: i64,
    pub end_column: i64,
}

#[php_impl]
impl Span {
    pub fn __construct(start_byte: i64, end_byte: i64, start_line: i64, start_column: i64, end_line: i64, end_column: i64) -> Self {
        Self { start_byte, end_byte, start_line, start_column, end_line, end_column }
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct ProcessResult {
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
    pub extractions: String,
}

#[php_impl]
impl ProcessResult {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for ProcessResult requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct FileMetrics {
    pub total_lines: i64,
    pub code_lines: i64,
    pub comment_lines: i64,
    pub blank_lines: i64,
    pub total_bytes: i64,
    pub node_count: i64,
    pub error_count: i64,
    pub max_depth: i64,
}

#[php_impl]
impl FileMetrics {
    pub fn __construct(
            total_lines: i64,
            code_lines: i64,
            comment_lines: i64,
            blank_lines: i64,
            total_bytes: i64,
            node_count: i64,
            error_count: i64,
            max_depth: i64,
        ) -> Self {
        Self { total_lines, code_lines, comment_lines, blank_lines, total_bytes, node_count, error_count, max_depth }
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct StructureItem {
    pub kind: String,
    pub name: Option<String>,
    pub visibility: Option<String>,
    pub span: Span,
    pub children: Vec<StructureItem>,
    pub decorators: Vec<String>,
    pub doc_comment: Option<String>,
    pub signature: Option<String>,
    pub body_span: Option<Span>,
}

#[php_impl]
impl StructureItem {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for StructureItem requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct CommentInfo {
    pub text: String,
    pub kind: String,
    pub span: Span,
    pub associated_node: Option<String>,
}

#[php_impl]
impl CommentInfo {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for CommentInfo requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct DocstringInfo {
    pub text: String,
    pub format: String,
    pub span: Span,
    pub associated_item: Option<String>,
    pub parsed_sections: Vec<DocSection>,
}

#[php_impl]
impl DocstringInfo {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for DocstringInfo requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct DocSection {
    pub kind: String,
    pub name: Option<String>,
    pub description: String,
}

#[php_impl]
impl DocSection {
    pub fn __construct(kind: String, description: String, name: Option<String>) -> Self {
        Self { kind, name, description }
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct ImportInfo {
    pub source: String,
    pub items: Vec<String>,
    pub alias: Option<String>,
    pub is_wildcard: bool,
    pub span: Span,
}

#[php_impl]
impl ImportInfo {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for ImportInfo requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct ExportInfo {
    pub name: String,
    pub kind: String,
    pub span: Span,
}

#[php_impl]
impl ExportInfo {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for ExportInfo requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct SymbolInfo {
    pub name: String,
    pub kind: String,
    pub span: Span,
    pub type_annotation: Option<String>,
    pub doc: Option<String>,
}

#[php_impl]
impl SymbolInfo {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for SymbolInfo requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct Diagnostic {
    pub message: String,
    pub severity: String,
    pub span: Span,
}

#[php_impl]
impl Diagnostic {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for Diagnostic requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct CodeChunk {
    pub content: String,
    pub start_byte: i64,
    pub end_byte: i64,
    pub start_line: i64,
    pub end_line: i64,
    pub metadata: ChunkContext,
}

#[php_impl]
impl CodeChunk {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for CodeChunk requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct ChunkContext {
    pub language: String,
    pub chunk_index: i64,
    pub total_chunks: i64,
    pub node_types: Vec<String>,
    pub context_path: Vec<String>,
    pub symbols_defined: Vec<String>,
    pub comments: Vec<CommentInfo>,
    pub docstrings: Vec<DocstringInfo>,
    pub has_error_nodes: bool,
}

#[php_impl]
impl ChunkContext {
    pub fn __construct() -> PhpResult<Self> {
        Err(PhpException::default("Not implemented: constructor for ChunkContext requires complex params".to_string()).into())
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct NodeInfo {
    pub kind: String,
    pub is_named: bool,
    pub start_byte: i64,
    pub end_byte: i64,
    pub start_row: i64,
    pub start_col: i64,
    pub end_row: i64,
    pub end_col: i64,
    pub named_child_count: i64,
    pub is_error: bool,
    pub is_missing: bool,
}

#[php_impl]
impl NodeInfo {
    pub fn __construct(
            kind: String,
            is_named: bool,
            start_byte: i64,
            end_byte: i64,
            start_row: i64,
            start_col: i64,
            end_row: i64,
            end_col: i64,
            named_child_count: i64,
            is_error: bool,
            is_missing: bool,
        ) -> Self {
        Self { kind, is_named, start_byte, end_byte, start_row, start_col, end_row, end_col, named_child_count, is_error, is_missing }
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct PackConfig {
    pub cache_dir: Option<String>,
    pub languages: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
}

#[php_impl]
impl PackConfig {
    pub fn __construct(cache_dir: Option<String>, languages: Option<Vec<String>>, groups: Option<Vec<String>>) -> Self {
        Self { cache_dir, languages, groups }
    }
}

#[derive(Clone, serde::Serialize)]
#[php_class]
pub struct ProcessConfig {
    pub language: String,
    pub structure: bool,
    pub imports: bool,
    pub exports: bool,
    pub comments: bool,
    pub docstrings: bool,
    pub symbols: bool,
    pub diagnostics: bool,
    pub chunk_max_size: Option<i64>,
    pub extractions: Option<String>,
}

#[php_impl]
impl ProcessConfig {
    pub fn __construct(
            language: String,
            structure: bool,
            imports: bool,
            exports: bool,
            comments: bool,
            docstrings: bool,
            symbols: bool,
            diagnostics: bool,
            chunk_max_size: Option<i64>,
            extractions: Option<String>,
        ) -> Self {
        Self { language, structure, imports, exports, comments, docstrings, symbols, diagnostics, chunk_max_size, extractions }
    }

    pub fn with_chunking(&self) -> ProcessConfig {
        todo!("Not auto-delegatable: with_chunking -- return type requires custom implementation")
    }

    pub fn all(&self) -> ProcessConfig {
        todo!("Not auto-delegatable: all -- return type requires custom implementation")
    }

    pub fn minimal(&self) -> ProcessConfig {
        todo!("Not auto-delegatable: minimal -- return type requires custom implementation")
    }

    pub fn default() -> ProcessConfig {
        todo!("Not auto-delegatable: default -- return type requires custom implementation")
    }
}

#[derive(Clone)]
#[php_class]
pub struct LanguageRegistry {
    inner: Arc<tree_sitter_language_pack::LanguageRegistry>,
}

#[php_impl]
impl LanguageRegistry {
    pub fn get_language(&self) -> PhpResult<Language> {
        Err(ext_php_rs::exception::PhpException::default("Not implemented: get_language".to_string()).into())
    }

    pub fn available_languages(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn has_language(&self) -> bool {
        false
    }

    pub fn language_count(&self) -> i64 {
        0
    }

    pub fn process(&self) -> PhpResult<ProcessResult> {
        Err(ext_php_rs::exception::PhpException::default("Not implemented: process".to_string()).into())
    }

    pub fn default() -> LanguageRegistry {
        todo!("Not auto-delegatable: default -- return type requires custom implementation")
    }
}

#[derive(Clone)]
#[php_class]
pub struct Language {
    inner: Arc<tree_sitter_language_pack::Language>,
}

#[php_impl]
impl Language {
}

#[derive(Clone)]
#[php_class]
pub struct Parser {
    inner: Arc<tree_sitter_language_pack::Parser>,
}

#[php_impl]
impl Parser {
}

#[derive(Clone)]
#[php_class]
pub struct Tree {
    inner: Arc<tree_sitter_language_pack::Tree>,
}

#[php_impl]
impl Tree {
}

// Error enum values
pub const ERROR_LANGUAGENOTFOUND: &str = "LanguageNotFound";
pub const ERROR_DYNAMICLOAD: &str = "DynamicLoad";
pub const ERROR_NULLLANGUAGEPOINTER: &str = "NullLanguagePointer";
pub const ERROR_PARSERSETUP: &str = "ParserSetup";
pub const ERROR_LOCKPOISONED: &str = "LockPoisoned";
pub const ERROR_CONFIG: &str = "Config";
pub const ERROR_PARSEFAILED: &str = "ParseFailed";
pub const ERROR_QUERYERROR: &str = "QueryError";
pub const ERROR_INVALIDRANGE: &str = "InvalidRange";
pub const ERROR_IO: &str = "Io";
pub const ERROR_JSON: &str = "Json";
pub const ERROR_TOML: &str = "Toml";
pub const ERROR_DOWNLOAD: &str = "Download";
pub const ERROR_CHECKSUMMISMATCH: &str = "ChecksumMismatch";

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

#[php_function]
pub fn detect_language_from_extension() -> Option<String> {
    None
}

#[php_function]
pub fn detect_language_from_path() -> Option<String> {
    None
}

#[php_function]
pub fn extension_ambiguity() -> Option<String> {
    None
}

#[php_function]
pub fn extension_ambiguity_json() -> Option<String> {
    None
}

#[php_function]
pub fn detect_language_from_content() -> Option<String> {
    None
}

#[php_function]
pub fn validate_extraction() -> PhpResult<ValidationResult> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: validate_extraction".to_string()).into())
}

#[php_function]
pub fn process() -> PhpResult<ProcessResult> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: process".to_string()).into())
}

#[php_function]
pub fn root_node_info() -> NodeInfo {
    todo!("Not auto-delegatable: root_node_info -- return type requires custom implementation")
}

#[php_function]
pub fn find_nodes_by_type() -> Vec<NodeInfo> {
    Vec::new()
}

#[php_function]
pub fn named_children_info() -> Vec<NodeInfo> {
    Vec::new()
}

#[php_function]
pub fn parse_string() -> PhpResult<Tree> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: parse_string".to_string()).into())
}

#[php_function]
pub fn tree_contains_node_type() -> bool {
    false
}

#[php_function]
pub fn tree_has_error_nodes() -> bool {
    false
}

#[php_function]
pub fn tree_to_sexp() -> String {
    String::from("[unimplemented: tree_to_sexp]")
}

#[php_function]
pub fn tree_error_count() -> i64 {
    0
}

#[php_function]
pub fn get_highlights_query() -> Option<String> {
    None
}

#[php_function]
pub fn get_injections_query() -> Option<String> {
    None
}

#[php_function]
pub fn get_locals_query() -> Option<String> {
    None
}

#[php_function]
pub fn split_code() -> Vec<String> {
    Vec::new()
}

#[php_function]
pub fn get_language() -> PhpResult<Language> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: get_language".to_string()).into())
}

#[php_function]
pub fn get_parser() -> PhpResult<Parser> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: get_parser".to_string()).into())
}

#[php_function]
pub fn available_languages() -> Vec<String> {
    Vec::new()
}

#[php_function]
pub fn has_language() -> bool {
    false
}

#[php_function]
pub fn language_count() -> i64 {
    0
}

#[php_function]
pub fn extract_patterns() -> PhpResult<ExtractionResult> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: extract_patterns".to_string()).into())
}

#[php_function]
pub fn init() -> PhpResult<()> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: init".to_string()).into())
}

#[php_function]
pub fn configure() -> PhpResult<()> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: configure".to_string()).into())
}

#[php_function]
pub fn download_all() -> PhpResult<i64> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: download_all".to_string()).into())
}

#[php_function]
pub fn manifest_languages() -> PhpResult<Vec<String>> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: manifest_languages".to_string()).into())
}

#[php_function]
pub fn downloaded_languages() -> Vec<String> {
    Vec::new()
}

#[php_function]
pub fn clean_cache() -> PhpResult<()> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: clean_cache".to_string()).into())
}

#[php_function]
pub fn cache_dir() -> PhpResult<String> {
    Err(ext_php_rs::exception::PhpException::default("Not implemented: cache_dir".to_string()).into())
}

impl From<PatternValidation> for tree_sitter_language_pack::PatternValidation {
    fn from(val: PatternValidation) -> Self {
        Self {
            valid: val.valid,
            capture_names: val.capture_names,
            pattern_count: val.pattern_count as usize,
            warnings: val.warnings,
            errors: val.errors,
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

impl From<CodeChunk> for tree_sitter_language_pack::CodeChunk {
    fn from(val: CodeChunk) -> Self {
        Self {
            content: val.content,
            start_byte: val.start_byte as usize,
            end_byte: val.end_byte as usize,
            start_line: val.start_line as usize,
            end_line: val.end_line as usize,
            metadata: val.metadata.into(),
        }
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
        Self {
            language: val.language,
            chunk_index: val.chunk_index as usize,
            total_chunks: val.total_chunks as usize,
            node_types: val.node_types,
            context_path: val.context_path,
            symbols_defined: val.symbols_defined,
            comments: val.comments,
            docstrings: val.docstrings,
            has_error_nodes: val.has_error_nodes,
        }
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
            comments: val.comments,
            docstrings: val.docstrings,
            has_error_nodes: val.has_error_nodes,
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
