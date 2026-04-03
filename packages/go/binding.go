package treesitterlanguagepackgo

/*
#cgo LDFLAGS: -lts_pack_ffi
#include "tslp.h"
import "C"
*/

import (
    "encoding/json"
    "fmt"
    "unsafe"
)

// lastError retrieves the last error from the FFI layer.
func lastError() error {
    code := int32(C.tslp_last_error_code())
    if code == 0 {
        return nil
    }
    ctx := C.tslp_last_error_context()
    message := C.GoString(ctx)
    C.tslp_free_string(ctx)
    return fmt.Errorf("[%d] %s", code, message)
}

// Errors that can occur when using the tree-sitter language pack.
//
// Covers language lookup failures, parse errors, query errors, and I/O issues.
// Feature-gated variants are included when `config`, `download`, or related
// features are enabled.
type Error string

const (
    ErrorLanguageNotFound Error = "language_not_found"
    ErrorDynamicLoad Error = "dynamic_load"
    ErrorNullLanguagePointer Error = "null_language_pointer"
    ErrorParserSetup Error = "parser_setup"
    ErrorLockPoisoned Error = "lock_poisoned"
    ErrorConfig Error = "config"
    ErrorParseFailed Error = "parse_failed"
    ErrorQueryError Error = "query_error"
    ErrorInvalidRange Error = "invalid_range"
    ErrorIo Error = "io"
    ErrorJson Error = "json"
    ErrorToml Error = "toml"
    ErrorDownload Error = "download"
    ErrorChecksumMismatch Error = "checksum_mismatch"
)


// Controls what data is captured for each query match.
type CaptureOutput string

const (
    // Capture only the matched text.
    CaptureOutputText CaptureOutput = "text"
    // Capture only the `NodeInfo`.
    CaptureOutputNode CaptureOutput = "node"
    // Capture both text and `NodeInfo` (default).
    CaptureOutputFull CaptureOutput = "full"
)


// The kind of structural item found in source code.
//
// Categorizes top-level and nested declarations such as functions, classes,
// structs, enums, traits, and more. Use [`Other`](StructureKind::Other) for
// language-specific constructs that do not fit a standard category.
type StructureKind string

const (
    StructureKindFunction StructureKind = "function"
    StructureKindMethod StructureKind = "method"
    StructureKindClass StructureKind = "class"
    StructureKindStruct StructureKind = "struct"
    StructureKindInterface StructureKind = "interface"
    StructureKindEnum StructureKind = "enum"
    StructureKindModule StructureKind = "module"
    StructureKindTrait StructureKind = "trait"
    StructureKindImpl StructureKind = "impl"
    StructureKindNamespace StructureKind = "namespace"
    StructureKindOther StructureKind = "other"
)


// The kind of a comment found in source code.
//
// Distinguishes between single-line comments, block (multi-line) comments,
// and documentation comments.
type CommentKind string

const (
    CommentKindLine CommentKind = "line"
    CommentKindBlock CommentKind = "block"
    CommentKindDoc CommentKind = "doc"
)


// The format of a docstring extracted from source code.
//
// Identifies the docstring convention used, which varies by language
// (e.g., Python triple-quoted strings, JSDoc, Rustdoc `///` comments).
type DocstringFormat string

const (
    DocstringFormatPythonTripleQuote DocstringFormat = "python_triple_quote"
    DocstringFormatJsDoc DocstringFormat = "js_doc"
    DocstringFormatRustdoc DocstringFormat = "rustdoc"
    DocstringFormatGoDoc DocstringFormat = "go_doc"
    DocstringFormatJavaDoc DocstringFormat = "java_doc"
    DocstringFormatOther DocstringFormat = "other"
)


// The kind of an export statement found in source code.
//
// Covers named exports, default exports, and re-exports from other modules.
type ExportKind string

const (
    ExportKindNamed ExportKind = "named"
    ExportKindDefault ExportKind = "default"
    ExportKindReExport ExportKind = "re_export"
)


// The kind of a symbol definition found in source code.
//
// Categorizes symbol definitions such as variables, constants, functions,
// classes, types, interfaces, enums, and modules.
type SymbolKind string

const (
    SymbolKindVariable SymbolKind = "variable"
    SymbolKindConstant SymbolKind = "constant"
    SymbolKindFunction SymbolKind = "function"
    SymbolKindClass SymbolKind = "class"
    SymbolKindType SymbolKind = "type"
    SymbolKindInterface SymbolKind = "interface"
    SymbolKindEnum SymbolKind = "enum"
    SymbolKindModule SymbolKind = "module"
    SymbolKindOther SymbolKind = "other"
)


// Severity level of a diagnostic produced during parsing.
//
// Used to classify parse errors, warnings, and informational messages
// found in the syntax tree.
type DiagnosticSeverity string

const (
    DiagnosticSeverityError DiagnosticSeverity = "error"
    DiagnosticSeverityWarning DiagnosticSeverity = "warning"
    DiagnosticSeverityInfo DiagnosticSeverity = "info"
)


// Defines a single extraction pattern and its configuration.
type ExtractionPattern struct {
    // The tree-sitter query string (S-expression).
    Query string `json:"query"`
    // What to include in each capture result.
    CaptureOutput CaptureOutput `json:"capture_output"`
    // Field names to extract from child nodes of each capture.
    // Maps a label to a tree-sitter field name used with `child_by_field_name`.
    ChildFields []string `json:"child_fields"`
    // Maximum number of matches to return. `None` means unlimited.
    MaxResults *uint `json:"max_results,omitempty"`
    // Restrict matches to a byte range `(start, end)`.
    ByteRange *string `json:"byte_range,omitempty"`
}


// Configuration for an extraction run against a single language.
type ExtractionConfig struct {
    // The language name (e.g., `"python"`).
    Language string `json:"language"`
    // Named patterns to run. Keys become the keys in `ExtractionResult::results`.
    Patterns string `json:"patterns"`
}


// A single captured node within a match.
type CaptureResult struct {
    // The capture name from the query (e.g., `"fn_name"`).
    Name string `json:"name"`
    // The `NodeInfo` snapshot, present when `CaptureOutput` is `Node` or `Full`.
    Node *NodeInfo `json:"node,omitempty"`
    // The matched source text, present when `CaptureOutput` is `Text` or `Full`.
    Text *string `json:"text,omitempty"`
    // Values of requested child fields, keyed by field name.
    ChildFields string `json:"child_fields"`
    // Byte offset where this capture starts in the source.
    StartByte uint `json:"start_byte"`
}


// A single query match containing one or more captures.
type MatchResult struct {
    // The pattern index within the query that produced this match.
    PatternIndex uint `json:"pattern_index"`
    // The captures for this match.
    Captures []CaptureResult `json:"captures"`
}


// Results for a single named pattern.
type PatternResult struct {
    // The individual matches.
    Matches []MatchResult `json:"matches"`
    // Total number of matches before `max_results` truncation.
    TotalCount uint `json:"total_count"`
}


// Complete extraction results for all patterns.
type ExtractionResult struct {
    // The language that was used.
    Language string `json:"language"`
    // Results keyed by pattern name.
    Results string `json:"results"`
}


// Validation information for a single pattern.
type PatternValidation struct {
    // Whether the pattern compiled successfully.
    Valid bool `json:"valid"`
    // Names of captures defined in the query.
    CaptureNames []string `json:"capture_names"`
    // Number of patterns in the query.
    PatternCount uint `json:"pattern_count"`
    // Non-fatal warnings (e.g., unused captures).
    Warnings []string `json:"warnings"`
    // Fatal errors (e.g., query syntax errors).
    Errors []string `json:"errors"`
}


// Validation results for an entire extraction config.
type ValidationResult struct {
    // Whether all patterns are valid.
    Valid bool `json:"valid"`
    // Per-pattern validation details.
    Patterns string `json:"patterns"`
}


// Byte and line/column range in source code.
//
// Represents both byte offsets (for slicing) and human-readable line/column
// positions (for display and diagnostics).
type Span struct {
    StartByte uint `json:"start_byte"`
    EndByte uint `json:"end_byte"`
    StartLine uint `json:"start_line"`
    StartColumn uint `json:"start_column"`
    EndLine uint `json:"end_line"`
    EndColumn uint `json:"end_column"`
}


// Complete analysis result from processing a source file.
//
// Contains metrics, structural analysis, imports/exports, comments,
// docstrings, symbols, diagnostics, and optionally chunked code segments.
// Fields are populated based on the [`crate::ProcessConfig`] flags.
//
// # Fields
//
// - `language` - The language used for parsing
// - `metrics` - Always computed: line counts, byte sizes, error counts
// - `structure` - Functions, classes, structs (when `config.structure = true`)
// - `imports` - Import statements (when `config.imports = true`)
// - `exports` - Export statements (when `config.exports = true`)
// - `comments` - Comments (when `config.comments = true`)
// - `docstrings` - Docstrings (when `config.docstrings = true`)
// - `symbols` - Symbol definitions (when `config.symbols = true`)
// - `diagnostics` - Parse errors (when `config.diagnostics = true`)
// - `chunks` - Chunked code segments (when `config.chunk_max_size` is set)
type ProcessResult struct {
    Language string `json:"language"`
    Metrics FileMetrics `json:"metrics"`
    Structure []StructureItem `json:"structure"`
    Imports []ImportInfo `json:"imports"`
    Exports []ExportInfo `json:"exports"`
    Comments []CommentInfo `json:"comments"`
    Docstrings []DocstringInfo `json:"docstrings"`
    Symbols []SymbolInfo `json:"symbols"`
    Diagnostics []Diagnostic `json:"diagnostics"`
    Chunks []CodeChunk `json:"chunks"`
    // Results of custom extraction patterns (when `config.extractions` is set).
    Extractions string `json:"extractions"`
}


// Aggregate metrics for a source file.
type FileMetrics struct {
    TotalLines uint `json:"total_lines"`
    CodeLines uint `json:"code_lines"`
    CommentLines uint `json:"comment_lines"`
    BlankLines uint `json:"blank_lines"`
    TotalBytes uint `json:"total_bytes"`
    NodeCount uint `json:"node_count"`
    ErrorCount uint `json:"error_count"`
    MaxDepth uint `json:"max_depth"`
}


// A structural item (function, class, struct, etc.) in source code.
type StructureItem struct {
    Kind StructureKind `json:"kind"`
    Name *string `json:"name,omitempty"`
    Visibility *string `json:"visibility,omitempty"`
    Span Span `json:"span"`
    Children []StructureItem `json:"children"`
    Decorators []string `json:"decorators"`
    DocComment *string `json:"doc_comment,omitempty"`
    Signature *string `json:"signature,omitempty"`
    BodySpan *Span `json:"body_span,omitempty"`
}


// A comment extracted from source code.
type CommentInfo struct {
    Text string `json:"text"`
    Kind CommentKind `json:"kind"`
    Span Span `json:"span"`
    AssociatedNode *string `json:"associated_node,omitempty"`
}


// A docstring extracted from source code.
type DocstringInfo struct {
    Text string `json:"text"`
    Format DocstringFormat `json:"format"`
    Span Span `json:"span"`
    AssociatedItem *string `json:"associated_item,omitempty"`
    ParsedSections []DocSection `json:"parsed_sections"`
}


// A section within a docstring (e.g., Args, Returns, Raises).
type DocSection struct {
    Kind string `json:"kind"`
    Name *string `json:"name,omitempty"`
    Description string `json:"description"`
}


// An import statement extracted from source code.
type ImportInfo struct {
    Source string `json:"source"`
    Items []string `json:"items"`
    Alias *string `json:"alias,omitempty"`
    IsWildcard bool `json:"is_wildcard"`
    Span Span `json:"span"`
}


// An export statement extracted from source code.
type ExportInfo struct {
    Name string `json:"name"`
    Kind ExportKind `json:"kind"`
    Span Span `json:"span"`
}


// A symbol (variable, function, type, etc.) extracted from source code.
type SymbolInfo struct {
    Name string `json:"name"`
    Kind SymbolKind `json:"kind"`
    Span Span `json:"span"`
    TypeAnnotation *string `json:"type_annotation,omitempty"`
    Doc *string `json:"doc,omitempty"`
}


// A diagnostic (syntax error, missing node, etc.) from parsing.
type Diagnostic struct {
    Message string `json:"message"`
    Severity DiagnosticSeverity `json:"severity"`
    Span Span `json:"span"`
}


// A chunk of source code with rich metadata.
type CodeChunk struct {
    Content string `json:"content"`
    StartByte uint `json:"start_byte"`
    EndByte uint `json:"end_byte"`
    StartLine uint `json:"start_line"`
    EndLine uint `json:"end_line"`
    Metadata ChunkContext `json:"metadata"`
}


// Metadata for a single chunk of source code.
type ChunkContext struct {
    Language string `json:"language"`
    ChunkIndex uint `json:"chunk_index"`
    TotalChunks uint `json:"total_chunks"`
    NodeTypes []string `json:"node_types"`
    ContextPath []string `json:"context_path"`
    SymbolsDefined []string `json:"symbols_defined"`
    Comments []CommentInfo `json:"comments"`
    Docstrings []DocstringInfo `json:"docstrings"`
    HasErrorNodes bool `json:"has_error_nodes"`
}


// Lightweight snapshot of a tree-sitter node's properties.
//
// Contains only primitive types for easy cross-language serialization.
// This is an owned type that can be passed across FFI boundaries, unlike
// `tree_sitter::Node` which borrows from the tree.
type NodeInfo struct {
    // The grammar type name (e.g., "function_definition", "identifier").
    Kind string `json:"kind"`
    // Whether this is a named node (vs anonymous like punctuation).
    IsNamed bool `json:"is_named"`
    // Start byte offset in source.
    StartByte uint `json:"start_byte"`
    // End byte offset in source.
    EndByte uint `json:"end_byte"`
    // Start row (zero-indexed).
    StartRow uint `json:"start_row"`
    // Start column (zero-indexed).
    StartCol uint `json:"start_col"`
    // End row (zero-indexed).
    EndRow uint `json:"end_row"`
    // End column (zero-indexed).
    EndCol uint `json:"end_col"`
    // Number of named children.
    NamedChildCount uint `json:"named_child_count"`
    // Whether this node is an ERROR node.
    IsError bool `json:"is_error"`
    // Whether this node is a MISSING node.
    IsMissing bool `json:"is_missing"`
}


// Configuration for the tree-sitter language pack.
//
// Controls cache directory and which languages to pre-download.
// Can be loaded from a TOML file, constructed programmatically,
// or passed as a dict/object from language bindings.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::PackConfig;
//
// let config = PackConfig {
// cache_dir: None,
// languages: Some(vec!["python".to_string(), "rust".to_string()]),
// groups: None,
// };
// ```
type PackConfig struct {
    // Override default cache directory.
    //
    // Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`
    CacheDir *string `json:"cache_dir,omitempty"`
    // Languages to pre-download on init.
    //
    // Each entry is a language name (e.g. `"python"`, `"rust"`).
    Languages *[]string `json:"languages,omitempty"`
    // Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`).
    Groups *[]string `json:"groups,omitempty"`
}


// Configuration for the `process()` function.
//
// Controls which analysis features are enabled and whether chunking is performed.
//
// # Examples
//
// ```
// use tree_sitter_language_pack::ProcessConfig;
//
// // Defaults: structure + imports + exports enabled
// let config = ProcessConfig::new("python");
//
// // With chunking
// let config = ProcessConfig::new("python").with_chunking(1000);
//
// // Everything enabled
// let config = ProcessConfig::new("python").all();
// ```
type ProcessConfig struct {
    // Language name (required).
    Language string `json:"language"`
    // Extract structural items (functions, classes, etc.). Default: true.
    Structure bool `json:"structure"`
    // Extract import statements. Default: true.
    Imports bool `json:"imports"`
    // Extract export statements. Default: true.
    Exports bool `json:"exports"`
    // Extract comments. Default: false.
    Comments bool `json:"comments"`
    // Extract docstrings. Default: false.
    Docstrings bool `json:"docstrings"`
    // Extract symbol definitions. Default: false.
    Symbols bool `json:"symbols"`
    // Include parse diagnostics. Default: false.
    Diagnostics bool `json:"diagnostics"`
    // Maximum chunk size in bytes. `None` disables chunking.
    ChunkMaxSize *uint `json:"chunk_max_size,omitempty"`
    // Custom extraction patterns to run against the parsed tree.
    // Keys become the keys in `ProcessResult::extractions`.
    Extractions *string `json:"extractions,omitempty"`
}


// Thread-safe registry of tree-sitter language parsers.
//
// Manages both statically compiled and dynamically loaded language grammars.
// Use [`LanguageRegistry::new()`] for the default registry, or access the
// global instance via the module-level convenience functions
// ([`crate::get_language`], [`crate::available_languages`], etc.).
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::{LanguageRegistry, ProcessConfig};
//
// let registry = LanguageRegistry::new();
// let langs = registry.available_languages();
// println!("Available: {:?}", langs);
//
// let config = ProcessConfig::new("python").all();
// let result = registry.process("def hello(): pass", &config).unwrap();
// println!("Structure: {:?}", result.structure);
// ```
type LanguageRegistry struct {
}


// Language is a type.
type Language struct {
}


// Parser is a type.
type Parser struct {
}


// Tree is a type.
type Tree struct {
}


// Detect language name from a file extension (without leading dot).
//
// Returns `None` for unrecognized extensions. The match is case-insensitive.
//
// ```
// use tree_sitter_language_pack::detect_language_from_extension;
// assert_eq!(detect_language_from_extension("py"), Some("python"));
// assert_eq!(detect_language_from_extension("RS"), Some("rust"));
// assert_eq!(detect_language_from_extension("xyz"), None);
// ```
func DetectLanguageFromExtension(ext string) **string {
    cExt := C.CString(ext)
    defer C.free(unsafe.Pointer(cExt))

    ptr := C.tslp_detect_language_from_extension(cExt)
    return unmarshalString(ptr)
}


// Detect language name from a file path.
//
// Extracts the file extension and looks it up. Returns `None` if the
// path has no extension or the extension is not recognized.
//
// ```
// use tree_sitter_language_pack::detect_language_from_path;
// assert_eq!(detect_language_from_path("src/main.rs"), Some("rust"));
// assert_eq!(detect_language_from_path("README.md"), Some("markdown"));
// assert_eq!(detect_language_from_path("Makefile"), None);
// ```
func DetectLanguageFromPath(path string) **string {
    cPath := C.CString(path)
    defer C.free(unsafe.Pointer(cPath))

    ptr := C.tslp_detect_language_from_path(cPath)
    return unmarshalString(ptr)
}


// Check if a file extension is ambiguous — i.e. it could reasonably belong to
// multiple languages.
//
// Returns `Some((assigned_language, alternatives))` if the extension is known
// to be ambiguous, where `assigned_language` is what [`detect_language_from_extension`]
// returns and `alternatives` lists other languages it could also belong to.
//
// Returns `None` if the extension is unambiguous or unrecognized.
//
// ```
// use tree_sitter_language_pack::extension_ambiguity;
// // .m is assigned to objc but could also be matlab
// if let Some((assigned, alternatives)) = extension_ambiguity("m") {
// assert_eq!(assigned, "objc");
// assert!(alternatives.contains(&"matlab"));
// }
// // .py is unambiguous
// assert!(extension_ambiguity("py").is_none());
// ```
func ExtensionAmbiguity(ext string) **string {
    cExt := C.CString(ext)
    defer C.free(unsafe.Pointer(cExt))

    ptr := C.tslp_extension_ambiguity(cExt)
    return unmarshalString(ptr)
}


// ExtensionAmbiguityJson calls the FFI function.
func ExtensionAmbiguityJson(ext string) **string {
    cExt := C.CString(ext)
    defer C.free(unsafe.Pointer(cExt))

    ptr := C.tslp_extension_ambiguity_json(cExt)
    return unmarshalString(ptr)
}


// Detect language name from file content using the shebang line (`#!`).
//
// Inspects only the first line of `content`. If it begins with `#!`, the
// interpreter name is extracted and mapped to a language name.
//
// Handles common patterns:
// - `#!/usr/bin/env python3` → `"python"`
// - `#!/bin/bash` → `"bash"`
// - `#!/usr/bin/env node` → `"javascript"`
//
// The `-S` flag accepted by some `env` implementations is skipped automatically.
// Version suffixes (e.g. `python3.11`, `ruby3.2`) are stripped before matching.
//
// Returns `None` when content does not start with `#!`, the shebang is
// malformed, or the interpreter is not recognised.
//
// ```
// use tree_sitter_language_pack::detect_language_from_content;
// assert_eq!(detect_language_from_content("#!/usr/bin/env python3\npass"), Some("python"));
// assert_eq!(detect_language_from_content("#!/bin/bash\necho hi"), Some("bash"));
// assert_eq!(detect_language_from_content("no shebang here"), None);
// ```
func DetectLanguageFromContent(content string) **string {
    cContent := C.CString(content)
    defer C.free(unsafe.Pointer(cContent))

    ptr := C.tslp_detect_language_from_content(cContent)
    return unmarshalString(ptr)
}


// Validate an extraction config without running it.
//
// Checks that the language exists and all query patterns compile. Returns
// detailed diagnostics per pattern.
//
// # Errors
//
// Returns an error if the language cannot be loaded.
func ValidateExtraction(config ExtractionConfig) *ValidationResult, error {
    jsonBytes, err := json.Marshal(config)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cConfig := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cConfig))

    ptr := C.tslp_validate_extraction(cConfig)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalValidationResult(ptr), nil
}


// Process source code: parse once, extract intelligence based on config, and return it.
func Process(source string, config ProcessConfig, registry LanguageRegistry) *ProcessResult, error {
    cSource := C.CString(source)
    defer C.free(unsafe.Pointer(cSource))

    jsonBytes, err := json.Marshal(config)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cConfig := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cConfig))

    jsonBytes, err := json.Marshal(registry)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cRegistry := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cRegistry))

    ptr := C.tslp_process(cSource, cConfig, cRegistry)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalProcessResult(ptr), nil
}


// Get a `NodeInfo` snapshot of the root node.
func RootNodeInfo(tree Tree) *NodeInfo {
    jsonBytes, err := json.Marshal(tree)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cTree := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cTree))

    ptr := C.tslp_root_node_info(cTree)
    return unmarshalNodeInfo(ptr)
}


// Find all nodes matching the given type name, returning their `NodeInfo`.
//
// Performs a depth-first traversal. Returns an empty vec if no matches.
func FindNodesByType(tree Tree, node_type string) *[]NodeInfo {
    jsonBytes, err := json.Marshal(tree)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cTree := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cTree))

    cNodeType := C.CString(node_type)
    defer C.free(unsafe.Pointer(cNodeType))

    ptr := C.tslp_find_nodes_by_type(cTree, cNodeType)
    return unmarshalListNodeInfo(ptr)
}


// Get `NodeInfo` for all named children of the root node.
//
// Useful for understanding the top-level structure of a file
// (e.g., list of function definitions, class declarations, imports).
func NamedChildrenInfo(tree Tree) *[]NodeInfo {
    jsonBytes, err := json.Marshal(tree)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cTree := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cTree))

    ptr := C.tslp_named_children_info(cTree)
    return unmarshalListNodeInfo(ptr)
}


// Parse source code with the named language, returning the syntax tree.
//
// Uses the global registry to look up the language by name.
//
// # Examples
//
// ```no_run
// let tree = tree_sitter_language_pack::parse::parse_string("python", b"def hello(): pass").unwrap();
// assert_eq!(tree.root_node().kind(), "module");
// ```
func ParseString(language string, source []byte) *Tree, error {
    cLanguage := C.CString(language)
    defer C.free(unsafe.Pointer(cLanguage))

    cSource := (*C.uchar)(unsafe.Pointer(&source[0]))

    ptr := C.tslp_parse_string(cLanguage, cSource)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalTree(ptr), nil
}


// Check whether any node in the tree matches the given type name.
//
// Performs a depth-first traversal using `TreeCursor`.
func TreeContainsNodeType(tree Tree, node_type string) *bool {
    jsonBytes, err := json.Marshal(tree)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cTree := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cTree))

    cNodeType := C.CString(node_type)
    defer C.free(unsafe.Pointer(cNodeType))

    ptr := C.tslp_tree_contains_node_type(cTree, cNodeType)
    return unmarshalBool(ptr)
}


// Check whether the tree contains any ERROR or MISSING nodes.
//
// Useful for determining if the parse was clean or had syntax errors.
func TreeHasErrorNodes(tree Tree) *bool {
    jsonBytes, err := json.Marshal(tree)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cTree := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cTree))

    ptr := C.tslp_tree_has_error_nodes(cTree)
    return unmarshalBool(ptr)
}


// Return the S-expression representation of the entire tree.
//
// This is the standard tree-sitter debug format, useful for logging,
// snapshot testing, and debugging grammars.
func TreeToSexp(tree Tree) *string {
    jsonBytes, err := json.Marshal(tree)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cTree := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cTree))

    ptr := C.tslp_tree_to_sexp(cTree)
    defer C.tslp_free_string(ptr)
    return unmarshalString(ptr)
}


// Count the number of ERROR and MISSING nodes in the tree.
//
// Returns 0 for a clean parse.
func TreeErrorCount(tree Tree) *uint {
    jsonBytes, err := json.Marshal(tree)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cTree := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cTree))

    ptr := C.tslp_tree_error_count(cTree)
    return unmarshalUsize(ptr)
}


// Get the highlights query for a language, if bundled.
//
// Returns the contents of `highlights.scm` as a static string, or `None`
// if no highlights query is bundled for this language.
//
// # Example
//
// ```
// use tree_sitter_language_pack::get_highlights_query;
//
// // Returns Some(...) for languages with bundled queries
// let query = get_highlights_query("python");
// // Returns None for languages without bundled highlights queries
// let missing = get_highlights_query("nonexistent_lang");
// assert!(missing.is_none());
// ```
func GetHighlightsQuery(language string) **string {
    cLanguage := C.CString(language)
    defer C.free(unsafe.Pointer(cLanguage))

    ptr := C.tslp_get_highlights_query(cLanguage)
    return unmarshalString(ptr)
}


// Get the injections query for a language, if bundled.
//
// Returns the contents of `injections.scm` as a static string, or `None`
// if no injections query is bundled for this language.
//
// # Example
//
// ```
// use tree_sitter_language_pack::get_injections_query;
//
// let query = get_injections_query("markdown");
// // Returns None for languages without bundled injections queries
// let missing = get_injections_query("nonexistent_lang");
// assert!(missing.is_none());
// ```
func GetInjectionsQuery(language string) **string {
    cLanguage := C.CString(language)
    defer C.free(unsafe.Pointer(cLanguage))

    ptr := C.tslp_get_injections_query(cLanguage)
    return unmarshalString(ptr)
}


// Get the locals query for a language, if bundled.
//
// Returns the contents of `locals.scm` as a static string, or `None`
// if no locals query is bundled for this language.
//
// # Example
//
// ```
// use tree_sitter_language_pack::get_locals_query;
//
// let query = get_locals_query("python");
// // Returns None for languages without bundled locals queries
// let missing = get_locals_query("nonexistent_lang");
// assert!(missing.is_none());
// ```
func GetLocalsQuery(language string) **string {
    cLanguage := C.CString(language)
    defer C.free(unsafe.Pointer(cLanguage))

    ptr := C.tslp_get_locals_query(cLanguage)
    return unmarshalString(ptr)
}


// Split source code into chunks using tree-sitter AST structure for intelligent boundaries.
// Returns a list of `(start_byte, end_byte)` ranges.
//
// The algorithm works by:
// 1. Walking the tree-sitter AST to collect all nodes with their depth.
// 2. Using depth as a semantic level: shallower nodes (functions, classes) are
// preferred split boundaries over deeper nodes (statements, expressions).
// 3. Greedily merging adjacent sections at the best semantic level that keeps
// each chunk under `max_chunk_size` bytes.
// 4. When no AST node boundary fits, falling back to line boundaries and
// ultimately to raw byte splits.
//
// The function never splits in the middle of a token/leaf node when an AST
// boundary is available.
//
// # Arguments
//
// * `source` - The full source code string.
// * `tree`   - A tree-sitter `Tree` previously parsed from `source`.
// * `max_chunk_size` - Maximum size in bytes for each chunk.
//
// # Returns
//
// A `Vec<(usize, usize)>` of `(start_byte, end_byte)` ranges covering the
// entire source. Ranges are non-overlapping, contiguous, and each range is
// at most `max_chunk_size` bytes (except when a single indivisible token
// exceeds that limit).
func SplitCode(source string, tree Tree, max_chunk_size uint) *[]string {
    cSource := C.CString(source)
    defer C.free(unsafe.Pointer(cSource))

    jsonBytes, err := json.Marshal(tree)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cTree := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cTree))

    ptr := C.tslp_split_code(cSource, cTree, cMaxChunkSize)
    return unmarshalListString(ptr)
}


// Get a tree-sitter [`Language`] by name using the global registry.
//
// Resolves language aliases (e.g., `"shell"` maps to `"bash"`).
// When the `download` feature is enabled (default), automatically downloads
// the parser from GitHub releases if not found locally.
//
// # Errors
//
// Returns [`Error::LanguageNotFound`] if the language is not recognized,
// or [`Error::Download`] if auto-download fails.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::get_language;
//
// let lang = get_language("python").unwrap();
// // Use the Language with a tree-sitter Parser
// let mut parser = tree_sitter::Parser::new();
// parser.set_language(&lang).unwrap();
// let tree = parser.parse("x = 1", None).unwrap();
// assert_eq!(tree.root_node().kind(), "module");
// ```
func GetLanguage(name string) *Language, error {
    cName := C.CString(name)
    defer C.free(unsafe.Pointer(cName))

    ptr := C.tslp_get_language(cName)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalLanguage(ptr), nil
}


// Get a tree-sitter [`Parser`] pre-configured for the given language.
//
// This is a convenience function that calls [`get_language`] and configures
// a new parser in one step.
//
// # Errors
//
// Returns [`Error::LanguageNotFound`] if the language is not recognized, or
// [`Error::ParserSetup`] if the language cannot be applied to the parser.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::get_parser;
//
// let mut parser = get_parser("rust").unwrap();
// let tree = parser.parse("fn main() {}", None).unwrap();
// assert!(!tree.root_node().has_error());
// ```
func GetParser(name string) *Parser, error {
    cName := C.CString(name)
    defer C.free(unsafe.Pointer(cName))

    ptr := C.tslp_get_parser(cName)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalParser(ptr), nil
}


// List all available language names (sorted, deduplicated, includes aliases).
//
// Returns names of both statically compiled and dynamically loadable languages,
// plus any configured aliases.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::available_languages;
//
// let langs = available_languages();
// for name in &langs {
// println!("{}", name);
// }
// ```
func AvailableLanguages() *[]string {
    ptr := C.tslp_available_languages()
    return unmarshalListString(ptr)
}


// Check if a language is available by name or alias.
//
// Returns `true` if the language can be loaded (statically compiled,
// dynamically available, or a known alias for one of these).
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::has_language;
//
// assert!(has_language("python"));
// assert!(has_language("shell")); // alias for "bash"
// assert!(!has_language("nonexistent_language"));
// ```
func HasLanguage(name string) *bool {
    cName := C.CString(name)
    defer C.free(unsafe.Pointer(cName))

    ptr := C.tslp_has_language(cName)
    return unmarshalBool(ptr)
}


// Return the number of available languages.
//
// Includes statically compiled languages, dynamically loadable languages,
// and aliases.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::language_count;
//
// let count = language_count();
// println!("{} languages available", count);
// ```
func LanguageCount() *uint {
    ptr := C.tslp_language_count()
    return unmarshalUsize(ptr)
}


// Run extraction patterns against source code.
//
// Convenience wrapper around [`extract::extract`].
//
// # Errors
//
// Returns an error if the language is not found, parsing fails, or a query
// pattern is invalid.
//
// # Example
//
// ```no_run
// use ahash::AHashMap;
// use tree_sitter_language_pack::{ExtractionConfig, ExtractionPattern, CaptureOutput, extract_patterns};
//
// let mut patterns = AHashMap::new();
// patterns.insert("fns".to_string(), ExtractionPattern {
// query: "(function_definition name: (identifier) @fn_name)".to_string(),
// capture_output: CaptureOutput::default(),
// child_fields: Vec::new(),
// max_results: None,
// byte_range: None,
// });
// let config = ExtractionConfig { language: "python".to_string(), patterns };
// let result = extract_patterns("def hello(): pass", &config).unwrap();
// ```
func ExtractPatterns(source string, config ExtractionConfig) *ExtractionResult, error {
    cSource := C.CString(source)
    defer C.free(unsafe.Pointer(cSource))

    jsonBytes, err := json.Marshal(config)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cConfig := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cConfig))

    ptr := C.tslp_extract_patterns(cSource, cConfig)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalExtractionResult(ptr), nil
}


// Initialize the language pack with the given configuration.
//
// Applies any custom cache directory, then downloads all languages and groups
// specified in the config. This is the recommended entry point when you want
// to pre-warm the cache before use.
//
// # Errors
//
// Returns an error if configuration cannot be applied or if downloads fail.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::{PackConfig, init};
//
// let config = PackConfig {
// cache_dir: None,
// languages: Some(vec!["python".to_string(), "rust".to_string()]),
// groups: None,
// };
// init(&config).unwrap();
// ```
func Init(config PackConfig) error {
    jsonBytes, err := json.Marshal(config)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cConfig := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cConfig))

    C.tslp_init(cConfig)
    return lastError()
}


// Apply download configuration without downloading anything.
//
// Use this to set a custom cache directory before the first call to
// [`get_language`] or any download function. Changing the cache dir
// after languages have been registered has no effect on already-loaded
// languages.
//
// # Errors
//
// Returns an error if the lock cannot be acquired.
//
// # Example
//
// ```no_run
// use std::path::PathBuf;
// use tree_sitter_language_pack::{PackConfig, configure};
//
// let config = PackConfig {
// cache_dir: Some(PathBuf::from("/tmp/my-parsers")),
// languages: None,
// groups: None,
// };
// configure(&config).unwrap();
// ```
func Configure(config PackConfig) error {
    jsonBytes, err := json.Marshal(config)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cConfig := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cConfig))

    C.tslp_configure(cConfig)
    return lastError()
}


// Download all available languages from the remote manifest.
//
// Returns the number of newly downloaded languages.
//
// # Errors
//
// Returns an error if the manifest cannot be fetched or a download fails.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::download_all;
//
// let count = download_all().unwrap();
// println!("Downloaded {} languages", count);
// ```
func DownloadAll() *uint, error {
    ptr := C.tslp_download_all()
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalUsize(ptr), nil
}


// Return all language names available in the remote manifest (248).
//
// Fetches (and caches) the remote manifest to discover the full list of
// downloadable languages. Use [`downloaded_languages`] to list what is
// already cached locally.
//
// # Errors
//
// Returns an error if the manifest cannot be fetched.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::manifest_languages;
//
// let langs = manifest_languages().unwrap();
// println!("{} languages available for download", langs.len());
// ```
func ManifestLanguages() *[]string, error {
    ptr := C.tslp_manifest_languages()
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalListString(ptr), nil
}


// Return languages that are already downloaded and cached locally.
//
// Does not perform any network requests. Returns an empty list if the
// cache directory does not exist or cannot be read.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::downloaded_languages;
//
// let langs = downloaded_languages();
// println!("{} languages already cached", langs.len());
// ```
func DownloadedLanguages() *[]string {
    ptr := C.tslp_downloaded_languages()
    return unmarshalListString(ptr)
}


// Delete all cached parser shared libraries.
//
// Resets the cache registration so the next call to [`get_language`] or
// a download function will re-register the (now empty) cache directory.
//
// # Errors
//
// Returns an error if the cache directory cannot be removed.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::clean_cache;
//
// clean_cache().unwrap();
// println!("Cache cleared");
// ```
func CleanCache() error {
    C.tslp_clean_cache()
    return lastError()
}


// Return the effective cache directory path.
//
// This is either the custom path set via [`configure`] / [`init`] or the
// default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`.
//
// # Errors
//
// Returns an error if the system cache directory cannot be determined.
//
// # Example
//
// ```no_run
// use tree_sitter_language_pack::cache_dir;
//
// let dir = cache_dir().unwrap();
// println!("Cache directory: {}", dir.display());
// ```
func CacheDir() *string, error {
    ptr := C.tslp_cache_dir()
    if err := lastError(); err != nil {
        if ptr != nil {
            C.tslp_free_string(ptr)
        }
        return nil, err
    }
    defer C.tslp_free_string(ptr)
    return unmarshalPath(ptr), nil
}


// Default is a method.
func (r *ProcessConfig) Default() *ProcessConfig {
    ptr := C.tslp_process_config_default (unsafe.Pointer(r), )
    return unmarshalProcessConfig(ptr)
}


// Enable chunking with the given maximum chunk size in bytes.
func (r *ProcessConfig) WithChunking(max_size uint) *ProcessConfig {
    ptr := C.tslp_process_config_with_chunking (unsafe.Pointer(r), cMaxSize)
    return unmarshalProcessConfig(ptr)
}


// Enable all analysis features.
func (r *ProcessConfig) All() *ProcessConfig {
    ptr := C.tslp_process_config_all (unsafe.Pointer(r), )
    return unmarshalProcessConfig(ptr)
}


// Disable all analysis features (only metrics computed).
func (r *ProcessConfig) Minimal() *ProcessConfig {
    ptr := C.tslp_process_config_minimal (unsafe.Pointer(r), )
    return unmarshalProcessConfig(ptr)
}


// Get a tree-sitter [`Language`] by name.
//
// Resolves aliases (e.g., `"shell"` -> `"bash"`, `"makefile"` -> `"make"`),
// then looks up the language in the static table. When the `dynamic-loading`
// feature is enabled, falls back to loading a shared library on demand.
//
// # Errors
//
// Returns [`Error::LanguageNotFound`] if the name (after alias resolution)
// does not match any known grammar.
func (r *LanguageRegistry) GetLanguage(name string) *Language, error {
    cName := C.CString(name)
    defer C.free(unsafe.Pointer(cName))

    ptr := C.tslp_language_registry_get_language (unsafe.Pointer(r), cName)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalLanguage(ptr), nil
}


// List all available language names, sorted and deduplicated.
//
// Includes statically compiled languages, dynamically loadable languages
// (if the `dynamic-loading` feature is enabled), and all configured aliases.
func (r *LanguageRegistry) AvailableLanguages() *[]string {
    ptr := C.tslp_language_registry_available_languages (unsafe.Pointer(r), )
    return unmarshalListString(ptr)
}


// Check whether a language is available by name or alias.
//
// Returns `true` if the language can be loaded, either from the static
// table or from a dynamic library on disk.
func (r *LanguageRegistry) HasLanguage(name string) *bool {
    cName := C.CString(name)
    defer C.free(unsafe.Pointer(cName))

    ptr := C.tslp_language_registry_has_language (unsafe.Pointer(r), cName)
    return unmarshalBool(ptr)
}


// Return the total number of available languages (including aliases).
func (r *LanguageRegistry) LanguageCount() *uint {
    ptr := C.tslp_language_registry_language_count (unsafe.Pointer(r), )
    return unmarshalUsize(ptr)
}


// Parse source code and extract file intelligence based on config in a single pass.
func (r *LanguageRegistry) Process(source string, config ProcessConfig) *ProcessResult, error {
    cSource := C.CString(source)
    defer C.free(unsafe.Pointer(cSource))

    jsonBytes, err := json.Marshal(config)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cConfig := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cConfig))

    ptr := C.tslp_language_registry_process (unsafe.Pointer(r), cSource, cConfig)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalProcessResult(ptr), nil
}


// Default is a method.
func (r *LanguageRegistry) Default() *LanguageRegistry {
    ptr := C.tslp_language_registry_default (unsafe.Pointer(r), )
    return unmarshalLanguageRegistry(ptr)
}
