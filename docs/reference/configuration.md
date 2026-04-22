---
title: "Configuration Reference"
---

## Configuration Reference

This page documents all configuration types and their defaults across all languages.

### ExtractionPattern

Defines a single extraction pattern and its configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `query` | `str` | — | The tree-sitter query string (S-expression). |
| `capture_output` | `CaptureOutput` | `CaptureOutput.FULL` | What to include in each capture result. |
| `child_fields` | `list[str]` | `[]` | Field names to extract from child nodes of each capture. Maps a label to a tree-sitter field name used with `child_by_field_name`. |
| `max_results` | `int | None` | `None` | Maximum number of matches to return. `None` means unlimited. |
| `byte_range` | `tuple[int, int] | None` | `None` | Restrict matches to a byte range `(start, end)`. |

---

### ExtractionConfig

Configuration for an extraction run against a single language.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | The language name (e.g., `"python"`). |
| `patterns` | `dict[str, ExtractionPattern]` | `{}` | Named patterns to run. Keys become the keys in `ExtractionResult.results`. |

---

### CaptureResult

A single captured node within a match.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The capture name from the query (e.g., `"fn_name"`). |
| `node` | `NodeInfo | None` | `None` | The `NodeInfo` snapshot, present when `CaptureOutput` is `Node` or `Full`. |
| `text` | `str | None` | `None` | The matched source text, present when `CaptureOutput` is `Text` or `Full`. |
| `child_fields` | `dict[str, str | None]` | `{}` | Values of requested child fields, keyed by field name. |
| `start_byte` | `int` | — | Byte offset where this capture starts in the source. |

---

### MatchResult

A single query match containing one or more captures.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pattern_index` | `int` | — | The pattern index within the query that produced this match. |
| `captures` | `list[CaptureResult]` | `[]` | The captures for this match. |

---

### PatternResult

Results for a single named pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `matches` | `list[MatchResult]` | `[]` | The individual matches. |
| `total_count` | `int` | — | Total number of matches before `max_results` truncation. |

---

### ExtractionResult

Complete extraction results for all patterns.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | The language that was used. |
| `results` | `dict[str, PatternResult]` | `{}` | Results keyed by pattern name. |

---

### PatternValidation

Validation information for a single pattern.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Whether the pattern compiled successfully. |
| `capture_names` | `list[str]` | `[]` | Names of captures defined in the query. |
| `pattern_count` | `int` | — | Number of patterns in the query. |
| `warnings` | `list[str]` | `[]` | Non-fatal warnings (e.g., unused captures). |
| `errors` | `list[str]` | `[]` | Fatal errors (e.g., query syntax errors). |

---

### ValidationResult

Validation results for an entire extraction config.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `valid` | `bool` | — | Whether all patterns are valid. |
| `patterns` | `dict[str, PatternValidation]` | `{}` | Per-pattern validation details. |

---

### Span

Byte and line/column range in source code.

Represents both byte offsets (for slicing) and human-readable line/column
positions (for display and diagnostics).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start_byte` | `int` | — | Start byte |
| `end_byte` | `int` | — | End byte |
| `start_line` | `int` | — | Start line |
| `start_column` | `int` | — | Start column |
| `end_line` | `int` | — | End line |
| `end_column` | `int` | — | End column |

---

### ProcessResult

Complete analysis result from processing a source file.

Contains metrics, structural analysis, imports/exports, comments,
docstrings, symbols, diagnostics, and optionally chunked code segments.
Fields are populated based on the `crate.ProcessConfig` flags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | Language |
| `metrics` | `FileMetrics` | — | Metrics (file metrics) |
| `structure` | `list[StructureItem]` | `[]` | Structure |
| `imports` | `list[ImportInfo]` | `[]` | Imports |
| `exports` | `list[ExportInfo]` | `[]` | Exports |
| `comments` | `list[CommentInfo]` | `[]` | Comments |
| `docstrings` | `list[DocstringInfo]` | `[]` | Docstrings |
| `symbols` | `list[SymbolInfo]` | `[]` | Symbols |
| `diagnostics` | `list[Diagnostic]` | `[]` | Diagnostics |
| `chunks` | `list[CodeChunk]` | `[]` | Text chunks for chunking/embedding |
| `extractions` | `dict[str, PatternResult]` | `{}` | Results of custom extraction patterns (when `config.extractions` is set). |

---

### FileMetrics

Aggregate metrics for a source file.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `total_lines` | `int` | — | Total lines |
| `code_lines` | `int` | — | Code lines |
| `comment_lines` | `int` | — | Comment lines |
| `blank_lines` | `int` | — | Blank lines |
| `total_bytes` | `int` | — | Total bytes |
| `node_count` | `int` | — | Number of node |
| `error_count` | `int` | — | Number of error |
| `max_depth` | `int` | — | Maximum depth |

---

### StructureItem

A structural item (function, class, struct, etc.) in source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `StructureKind` | `StructureKind.FUNCTION` | Kind (structure kind) |
| `name` | `str | None` | `None` | The name |
| `visibility` | `str | None` | `None` | Visibility |
| `span` | `Span` | — | Span (span) |
| `children` | `list[StructureItem]` | `[]` | Children |
| `decorators` | `list[str]` | `[]` | Decorators |
| `doc_comment` | `str | None` | `None` | Doc comment |
| `signature` | `str | None` | `None` | Signature |
| `body_span` | `Span | None` | `None` | Body span (span) |

---

### CommentInfo

A comment extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `str` | — | Text |
| `kind` | `CommentKind` | `CommentKind.LINE` | Kind (comment kind) |
| `span` | `Span` | — | Span (span) |
| `associated_node` | `str | None` | `None` | Associated node |

---

### DocstringInfo

A docstring extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `str` | — | Text |
| `format` | `DocstringFormat` | `DocstringFormat.PYTHON_TRIPLE_QUOTE` | Format (docstring format) |
| `span` | `Span` | — | Span (span) |
| `associated_item` | `str | None` | `None` | Associated item |
| `parsed_sections` | `list[DocSection]` | `[]` | Parsed sections |

---

### DocSection

A section within a docstring (e.g., Args, Returns, Raises).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `str` | — | Kind |
| `name` | `str | None` | `None` | The name |
| `description` | `str` | — | Human-readable description |

---

### ImportInfo

An import statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `str` | — | Source |
| `items` | `list[str]` | `[]` | Items |
| `alias` | `str | None` | `None` | Alias |
| `is_wildcard` | `bool` | — | Whether wildcard |
| `span` | `Span` | — | Span (span) |

---

### ExportInfo

An export statement extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `kind` | `ExportKind` | `ExportKind.NAMED` | Kind (export kind) |
| `span` | `Span` | — | Span (span) |

---

### SymbolInfo

A symbol (variable, function, type, etc.) extracted from source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `kind` | `SymbolKind` | `SymbolKind.VARIABLE` | Kind (symbol kind) |
| `span` | `Span` | — | Span (span) |
| `type_annotation` | `str | None` | `None` | Type annotation |
| `doc` | `str | None` | `None` | Doc |

---

### Diagnostic

A diagnostic (syntax error, missing node, etc.) from parsing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `str` | — | Message |
| `severity` | `DiagnosticSeverity` | `DiagnosticSeverity.ERROR` | Severity (diagnostic severity) |
| `span` | `Span` | — | Span (span) |

---

### CodeChunk

A chunk of source code with rich metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The extracted text content |
| `start_byte` | `int` | — | Start byte |
| `end_byte` | `int` | — | End byte |
| `start_line` | `int` | — | Start line |
| `end_line` | `int` | — | End line |
| `metadata` | `ChunkContext` | — | Document metadata |

---

### ChunkContext

Metadata for a single chunk of source code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | — | Language |
| `chunk_index` | `int` | — | Chunk index |
| `total_chunks` | `int` | — | Total chunks |
| `node_types` | `list[str]` | `[]` | Node types |
| `context_path` | `list[str]` | `[]` | Context path |
| `symbols_defined` | `list[str]` | `[]` | Symbols defined |
| `comments` | `list[CommentInfo]` | `[]` | Comments |
| `docstrings` | `list[DocstringInfo]` | `[]` | Docstrings |
| `has_error_nodes` | `bool` | — | Whether error nodes |

---

### NodeInfo

Lightweight snapshot of a tree-sitter node's properties.

Contains only primitive types for easy cross-language serialization.
This is an owned type that can be passed across FFI boundaries, unlike
`tree_sitter.Node` which borrows from the tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `Str` | — | The grammar type name (e.g., "function_definition", "identifier"). |
| `is_named` | `bool` | — | Whether this is a named node (vs anonymous like punctuation). |
| `start_byte` | `int` | — | Start byte offset in source. |
| `end_byte` | `int` | — | End byte offset in source. |
| `start_row` | `int` | — | Start row (zero-indexed). |
| `start_col` | `int` | — | Start column (zero-indexed). |
| `end_row` | `int` | — | End row (zero-indexed). |
| `end_col` | `int` | — | End column (zero-indexed). |
| `named_child_count` | `int` | — | Number of named children. |
| `is_error` | `bool` | — | Whether this node is an ERROR node. |
| `is_missing` | `bool` | — | Whether this node is a MISSING node. |

---

### PackConfig

Configuration for the tree-sitter language pack.

Controls cache directory and which languages to pre-download.
Can be loaded from a TOML file, constructed programmatically,
or passed as a dict/object from language bindings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `str | None` | `None` | Override default cache directory. Default: `~/.cache/tree-sitter-language-pack/v{version}/libs/` |
| `languages` | `list[str] | None` | `[]` | Languages to pre-download on init. Each entry is a language name (e.g. `"python"`, `"rust"`). |
| `groups` | `list[str] | None` | `[]` | Language groups to pre-download (e.g. `"web"`, `"systems"`, `"scripting"`). |

---

### ProcessConfig

Configuration for the `process()` function.

Controls which analysis features are enabled and whether chunking is performed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `Str` | — | Language name (required). |
| `structure` | `bool` | `True` | Extract structural items (functions, classes, etc.). Default: true. |
| `imports` | `bool` | `True` | Extract import statements. Default: true. |
| `exports` | `bool` | `True` | Extract export statements. Default: true. |
| `comments` | `bool` | `False` | Extract comments. Default: false. |
| `docstrings` | `bool` | `False` | Extract docstrings. Default: false. |
| `symbols` | `bool` | `False` | Extract symbol definitions. Default: false. |
| `diagnostics` | `bool` | `False` | Include parse diagnostics. Default: false. |
| `chunk_max_size` | `int | None` | `None` | Maximum chunk size in bytes. `None` disables chunking. |
| `extractions` | `dict[str, ExtractionPattern] | None` | `None` | Custom extraction patterns to run against the parsed tree. Keys become the keys in `ProcessResult.extractions`. |

---

### QueryMatch

A single match from a tree-sitter query, with captured nodes.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pattern_index` | `int` | — | The pattern index that matched (position in the query string). |
| `captures` | `list[tuple[CowStatic, Str, NodeInfo]]` | `[]` | Captures: list of (capture_name, node_info) pairs. |

---

### Config

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language_pack` | `LanguagePackConfig` | — | Language pack (language pack config) |
| `languages` | `LanguagesConfig` | — | Languages (languages config) |

---

### LanguagePackConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cache_dir` | `str | None` | `None` | Cache dir |
| `definitions` | `str | None` | `None` | Definitions |

---

### LanguagesConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `include` | `list[str]` | `[]` | Include |
| `exclude` | `list[str]` | `[]` | Exclude |

---
