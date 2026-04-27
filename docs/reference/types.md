---
title: "Types Reference"
---

## Types Reference

All types defined by the library, grouped by category. Types are shown using Rust as the canonical representation.

### Result Types

#### CaptureResult

A single captured node within a match.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The capture name from the query (e.g., `"fn_name"`). |
| `node` | `Option<NodeInfo>` | `Default::default()` | The `NodeInfo` snapshot, present when `CaptureOutput` is `Node` or `Full`. |
| `text` | `Option<String>` | `Default::default()` | The matched source text, present when `CaptureOutput` is `Text` or `Full`. |
| `child_fields` | `String` | — | Values of requested child fields, keyed by field name. |
| `start_byte` | `usize` | — | Byte offset where this capture starts in the source. |

---

#### MatchResult

A single query match containing one or more captures.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pattern_index` | `usize` | — | The pattern index within the query that produced this match. |
| `captures` | `Vec<CaptureResult>` | `vec![]` | The captures for this match. |

---

#### PatternResult

Results for a single named pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `matches` | `Vec<MatchResult>` | `vec![]` | The individual matches. |
| `total_count` | `usize` | — | Total number of matches before `max_results` truncation. |

---

#### ExtractionResult

Complete extraction results for all patterns.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | The language that was used. |
| `results` | `String` | — | Results keyed by pattern name. |

---

#### ValidationResult

Validation results for an entire extraction config.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Whether all patterns are valid. |
| `patterns` | `String` | — | Per-pattern validation details. |

---

#### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `crate.ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language |
| `metrics` | `FileMetrics` | — | Metrics (file metrics) |
| `structure` | `Vec<StructureItem>` | `vec![]` | Structure |
| `imports` | `Vec<ImportInfo>` | `vec![]` | Imports |
| `exports` | `Vec<ExportInfo>` | `vec![]` | Exports |
| `comments` | `Vec<CommentInfo>` | `vec![]` | Comments |
| `docstrings` | `Vec<DocstringInfo>` | `vec![]` | Docstrings |
| `symbols` | `Vec<SymbolInfo>` | `vec![]` | Symbols |
| `diagnostics` | `Vec<Diagnostic>` | `vec![]` | Diagnostics |
| `chunks` | `Vec<CodeChunk>` | `vec![]` | Text chunks for chunking/embedding |
| `extractions` | `String` | — | Results of custom extraction patterns (when `config.extractions` is set). |

---

### Configuration Types

See [Configuration Reference](configuration.md) for detailed defaults and language-specific representations.

#### ExtractionPattern

Defines a single extraction pattern and its configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `query` | `String` | — | The tree-sitter query string (S-expression). |
| `capture_output` | `CaptureOutput` | `CaptureOutput::Full` | What to include in each capture result. |
| `child_fields` | `Vec<String>` | `vec![]` | Field names to extract from child nodes of each capture. Maps a label to a tree-sitter field name used with `child_by_field_name`. |
| `max_results` | `Option<usize>` | `Default::default()` | Maximum number of matches to return. `None` means unlimited. |
| `byte_range` | `Vec<usize>` | `vec![]` | Restrict matches to a byte range `(start, end)`. |

---

#### ExtractionConfig

Configuration for an extraction run against a single language.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | The language name (e.g., `"python"`). |
| `patterns` | `String` | — | Named patterns to run. Keys become the keys in `ExtractionResult.results`. |

---

#### PatternValidation

Validation information for a single pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Whether the pattern compiled successfully. |
| `capture_names` | `Vec<String>` | `vec![]` | Names of captures defined in the query. |
| `pattern_count` | `usize` | — | Number of patterns in the query. |
| `warnings` | `Vec<String>` | `vec![]` | Non-fatal warnings (e.g., unused captures). |
| `errors` | `Vec<String>` | `vec![]` | Fatal errors (e.g., query syntax errors). |

---

#### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `usize` | — | Start byte |
| `end_byte` | `usize` | — | End byte |
| `start_line` | `usize` | — | Start line |
| `start_column` | `usize` | — | Start column |
| `end_line` | `usize` | — | End line |
| `end_column` | `usize` | — | End column |

---

#### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `usize` | — | Total lines |
| `code_lines` | `usize` | — | Code lines |
| `comment_lines` | `usize` | — | Comment lines |
| `blank_lines` | `usize` | — | Blank lines |
| `total_bytes` | `usize` | — | Total bytes |
| `node_count` | `usize` | — | Number of nodes |
| `error_count` | `usize` | — | Number of errors |
| `max_depth` | `usize` | — | Maximum depth |

---

#### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind::Function` | Kind (structure kind) |
| `name` | `Option<String>` | `Default::default()` | The name |
| `visibility` | `Option<String>` | `Default::default()` | Visibility |
| `span` | `Span` | — | Span (span) |
| `children` | `Vec<StructureItem>` | `vec![]` | Children |
| `decorators` | `Vec<String>` | `vec![]` | Decorators |
| `doc_comment` | `Option<String>` | `Default::default()` | Doc comment |
| `signature` | `Option<String>` | `Default::default()` | Signature |
| `body_span` | `Option<Span>` | `Default::default()` | Body span (span) |

---

#### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `kind` | `CommentKind` | `CommentKind::Line` | Kind (comment kind) |
| `span` | `Span` | — | Span (span) |
| `associated_node` | `Option<String>` | `Default::default()` | Associated node |

---

#### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `format` | `DocstringFormat` | `DocstringFormat::PythonTripleQuote` | Format (docstring format) |
| `span` | `Span` | — | Span (span) |
| `associated_item` | `Option<String>` | `Default::default()` | Associated item |
| `parsed_sections` | `Vec<DocSection>` | `vec![]` | Parsed sections |

---

#### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | Kind |
| `name` | `Option<String>` | `Default::default()` | The name |
| `description` | `String` | — | Human-readable description |

---

#### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `String` | — | Source |
| `items` | `Vec<String>` | `vec![]` | Items |
| `alias` | `Option<String>` | `Default::default()` | Alias |
| `is_wildcard` | `bool` | — | Whether wildcard |
| `span` | `Span` | — | Span (span) |

---

#### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `kind` | `ExportKind` | `ExportKind::Named` | Kind (export kind) |
| `span` | `Span` | — | Span (span) |

---

#### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `kind` | `SymbolKind` | `SymbolKind::Variable` | Kind (symbol kind) |
| `span` | `Span` | — | Span (span) |
| `type_annotation` | `Option<String>` | `Default::default()` | Type annotation |
| `doc` | `Option<String>` | `Default::default()` | Doc |

---

#### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Message |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity::Error` | Severity (diagnostic severity) |
| `span` | `Span` | — | Span (span) |

---

#### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `start_byte` | `usize` | — | Start byte |
| `end_byte` | `usize` | — | End byte |
| `start_line` | `usize` | — | Start line |
| `end_line` | `usize` | — | End line |
| `metadata` | `ChunkContext` | — | Document metadata |

---

#### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language |
| `chunk_index` | `usize` | — | Chunk index |
| `total_chunks` | `usize` | — | Total chunks |
| `node_types` | `Vec<String>` | `vec![]` | Node types |
| `context_path` | `Vec<String>` | `vec![]` | Context path |
| `symbols_defined` | `Vec<String>` | `vec![]` | Symbols defined |
| `comments` | `Vec<CommentInfo>` | `vec![]` | Comments |
| `docstrings` | `Vec<DocstringInfo>` | `vec![]` | Docstrings |
| `has_error_nodes` | `bool` | — | Whether error nodes |

---

#### NodeInfo

Lightweight snapshot of a tree-sitter node's properties.

Contains only primitive types for easy cross-language serialization.
This is an owned type that can be passed across FFI boundaries, unlike
`tree_sitter.Node` which borrows from the tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `String` | — | The grammar type name (e.g., "function_definition", "identifier"). |
| `is_named` | `bool` | — | Whether this is a named node (vs anonymous like punctuation). |
| `start_byte` | `usize` | — | Start byte offset in source. |
| `end_byte` | `usize` | — | End byte offset in source. |
| `start_row` | `usize` | — | Start row (zero-indexed). |
| `start_col` | `usize` | — | Start column (zero-indexed). |
| `end_row` | `usize` | — | End row (zero-indexed). |
| `end_col` | `usize` | — | End column (zero-indexed). |
| `named_child_count` | `usize` | — | Number of named children. |
| `is_error` | `bool` | — | Whether this node is an ERROR node. |
| `is_missing` | `bool` | — | Whether this node is a MISSING node. |

---

#### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `Option<PathBuf>` | `Default::default()` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `Vec<String>` | `vec![]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `Vec<String>` | `vec![]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

#### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `String` | — | Language name (required). |
| `structure` | `bool` | `true` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `bool` | `true` | Extract import statements. Default: true. |
| `exports` | `bool` | `true` | Extract export statements. Default: true. |
| `comments` | `bool` | `false` | Extract comments. Default: false. |
| `docstrings` | `bool` | `false` | Extract docstrings. Default: false. |
| `symbols` | `bool` | `false` | Extract symbol definitions. Default: false. |
| `diagnostics` | `bool` | `false` | Include parse diagnostics. Default: false. |
| `chunk_max_size` | `Option<usize>` | `None` | Maximum chunk size in bytes. `None` disables chunking. |
| `extractions` | `Option<String>` | `None` | Custom extraction patterns to run against the parsed tree. Keys become the keys in `ProcessResult.extractions`. |

---

#### QueryMatch

A single match from a tree-sitter query, with captured nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pattern_index` | `usize` | — | The pattern index that matched (position in the query string). |
| `captures` | `Vec<String>` | `vec![]` | Captures: list of (capture_name, node_info) pairs. |

---

#### LanguageRegistry

Thread-safe registry of tree-sitter language parsers.

Manages both statically compiled and dynamically loaded language grammars.
Use `LanguageRegistry.new()` for the default registry, or access the
global instance via the module-level convenience functions
(`crate.get_language`, `crate.available_languages`, etc.).

*Opaque type — fields are not directly accessible.*

---

### Other Types

#### ParserManifest

Manifest describing available parser downloads for a specific version.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `version` | `String` | — | Version string |
| `platforms` | `HashMap<String, PlatformBundle>` | — | Platforms |
| `languages` | `HashMap<String, LanguageInfo>` | — | Languages |
| `groups` | `HashMap<String, Vec<String>>` | — | Groups |

---

#### PlatformBundle

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `sha256` | `String` | — | Sha256 |
| `size` | `u64` | — | Size in bytes |

---

#### LanguageInfo

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `group` | `String` | — | Group |
| `size` | `u64` | — | Size in bytes |

---

#### DownloadManager

Manages downloading and caching of pre-built parser shared libraries.

*Opaque type — fields are not directly accessible.*

---

#### Language

*Opaque type — fields are not directly accessible.*

---

#### Parser

*Opaque type — fields are not directly accessible.*

---

#### Tree

*Opaque type — fields are not directly accessible.*

---
