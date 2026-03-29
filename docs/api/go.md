---
description: "Go API reference for tree-sitter-language-pack"
---

# Go API Reference

## Installation

```bash
go get github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go
```

Import path:

```go
import tspack "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
```

## Quick Start

```go
package main

import (
    "fmt"
    "log"

    tspack "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
)

func main() {
    reg, err := tspack.NewRegistry()
    if err != nil {
        log.Fatal(err)
    }
    defer reg.Close()

    // Check available languages
    langs := reg.AvailableLanguages()
    fmt.Printf("%d languages available\n", len(langs))

    // Parse source code
    tree, err := reg.ParseString("python", "def hello(): pass")
    if err != nil {
        log.Fatal(err)
    }
    defer tree.Close()

    rootType, _ := tree.RootNodeType()
    fmt.Println(rootType) // "module"

    // Extract code intelligence
    config := tspack.NewProcessConfig("python")
    result, err := reg.Process("def hello(): pass", config)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Functions: %d\n", len(result.Metadata.Structure))
}
```

## Registry

### `NewRegistry() (*Registry, error)`

Create a new language registry containing all available tree-sitter grammars. The registry is safe for concurrent use from multiple goroutines. Must be closed with `Close()` when no longer needed.

**Returns:** *Registry, error

**Example:**

```go
reg, err := tspack.NewRegistry()
if err != nil {
    log.Fatal(err)
}
defer reg.Close()
```

### `Registry.Close()`

Explicitly free the underlying C registry. Safe to call multiple times. After closing, all other methods return errors or zero values.

### `Registry.GetLanguage(name string) (unsafe.Pointer, error)`

Return a pointer to the TSLanguage for the given language name. The returned `unsafe.Pointer` can be cast to the appropriate type by consumers (e.g., go-tree-sitter's Language type). The pointer remains valid for the lifetime of the Registry.

**Parameters:**

- `name` (string): Language name

**Returns:** unsafe.Pointer, error

**Example:**

```go
langPtr, err := reg.GetLanguage("python")
if err != nil {
    log.Fatal(err)
}
// Pass langPtr to a tree-sitter Go wrapper
```

### `Registry.LanguageCount() int`

Return the number of available languages. Returns 0 if the registry is closed.

### `Registry.LanguageNameAt(index int) (string, error)`

Return the language name at the given index. Valid indices are `[0, LanguageCount())`.

### `Registry.HasLanguage(name string) bool`

Check whether the registry contains a grammar for the named language. Returns false if the registry is closed.

**Parameters:**

- `name` (string): Language name

**Returns:** bool

### `Registry.AvailableLanguages() []string`

Return a slice of all language names in the registry. Returns nil if the registry is closed.

**Returns:** []string

### `Registry.ParseString(language, source string) (*Tree, error)`

Parse the given source code using the named language and return a Tree handle. The caller must call `Tree.Close()` when done.

**Parameters:**

- `language` (string): Language name
- `source` (string): Source code

**Returns:** *Tree, error

**Example:**

```go
tree, err := reg.ParseString("python", "def foo(): pass")
if err != nil {
    log.Fatal(err)
}
defer tree.Close()
```

### `Registry.Process(source string, config ProcessConfig) (*ProcessResult, error)`

Extract file intelligence from source code using a `ProcessConfig`. Returns a typed `ProcessResult` with deserialized metadata and chunks.

**Parameters:**

- `source` (string): Source code
- `config` (ProcessConfig): Configuration specifying language and extraction features

**Returns:** *ProcessResult, error

**Example:**

```go
config := tspack.NewProcessConfig("python")
result, err := reg.Process("def hello(): pass", config)
if err != nil {
    log.Fatal(err)
}
fmt.Printf("Functions: %d\n", len(result.Metadata.Structure))
```

## Tree

### `Tree.Close()`

Free the underlying C tree. Safe to call multiple times.

### `Tree.RootNodeType() (string, error)`

Return the type name of the root node.

### `Tree.RootChildCount() (int, error)`

Return the number of named children of the root node.

### `Tree.ContainsNodeType(nodeType string) (bool, error)`

Check whether any node in the tree has the given type name.

### `Tree.HasErrorNodes() (bool, error)`

Check whether the tree contains any ERROR or MISSING nodes.

## Language Detection

### `DetectLanguage(path string) string`

Detect a language name from a file path or extension. Returns an empty string if not recognized.

**Parameters:**

- `path` (string): File path or extension

**Returns:** string

### `DetectLanguageFromExtension(ext string) string`

Detect a language name from a bare file extension (without the leading dot). Returns an empty string if not recognized.

**Parameters:**

- `ext` (string): File extension (e.g., `"rs"`, `"py"`)

**Returns:** string

**Example:**

```go
lang := tspack.DetectLanguageFromExtension("rs")
// lang == "rust"
```

### `DetectLanguageFromPath(path string) string`

Detect a language name from a file path. Returns an empty string if not recognized.

**Parameters:**

- `path` (string): File path

**Returns:** string

**Example:**

```go
lang := tspack.DetectLanguageFromPath("/home/user/project/main.py")
// lang == "python"
```

### `DetectLanguageFromContent(content string) string`

Detect a language name from file content using shebang-based detection. Returns an empty string if no shebang is recognized.

**Parameters:**

- `content` (string): File content

**Returns:** string

### `ExtensionAmbiguity(ext string) (*ExtensionAmbiguityResult, error)`

Return ambiguity information for the given file extension. Returns nil if the extension is not ambiguous.

**Returns:** *ExtensionAmbiguityResult, error

```go
type ExtensionAmbiguityResult struct {
    Assigned     string   `json:"assigned"`
    Alternatives []string `json:"alternatives"`
}
```

## Bundled Queries

### `GetHighlightsQuery(language string) string`

Return the bundled highlights query for the given language. Returns an empty string if not available.

### `GetInjectionsQuery(language string) string`

Return the bundled injections query for the given language. Returns an empty string if not available.

### `GetLocalsQuery(language string) string`

Return the bundled locals query for the given language. Returns an empty string if not available.

## Download Management

### `Init(configJSON string) error`

Initialize the language pack with configuration. `configJSON` is a JSON string with optional fields: `"cache_dir"` (string), `"languages"` (array), `"groups"` (array).

**Parameters:**

- `configJSON` (string): JSON configuration string

**Returns:** error

**Example:**

```go
err := tspack.Init(`{"languages": ["python", "rust"], "cache_dir": "/opt/ts-pack"}`)
if err != nil {
    log.Fatal(err)
}
```

### `Configure(configJSON string) error`

Configure the language pack cache directory without downloading. `configJSON` is a JSON string with an optional `"cache_dir"` field.

**Parameters:**

- `configJSON` (string): JSON configuration string

**Returns:** error

**Example:**

```go
err := tspack.Configure(`{"cache_dir": "/data/ts-pack"}`)
if err != nil {
    log.Fatal(err)
}
```

### `Download(languages []string) (int, error)`

Download specific languages to the cache. Returns the number of newly downloaded languages.

**Parameters:**

- `languages` ([]string): Language names to download

**Returns:** int, error

**Example:**

```go
count, err := tspack.Download([]string{"python", "rust", "typescript"})
if err != nil {
    log.Fatal(err)
}
fmt.Printf("Downloaded %d new languages\n", count)
```

### `DownloadAll() (int, error)`

Download all available languages from the remote manifest. Returns the number of newly downloaded languages.

**Returns:** int, error

### `ManifestLanguages() ([]string, error)`

Return all language names available in the remote manifest.

**Returns:** []string, error

### `DownloadedLanguages() ([]string, error)`

Return all languages that are already downloaded and cached locally.

**Returns:** []string, error

### `CleanCache() error`

Delete all cached parser shared libraries.

**Returns:** error

### `CacheDir() (string, error)`

Return the effective cache directory path.

**Returns:** string, error

## Types

### `ProcessConfig`

Configuration specifying what to extract from source code.

```go
type ProcessConfig struct {
    Language     string `json:"language"`
    Structure    bool   `json:"structure"`
    Imports      bool   `json:"imports"`
    Exports      bool   `json:"exports"`
    Comments     bool   `json:"comments"`
    Docstrings   bool   `json:"docstrings"`
    Symbols      bool   `json:"symbols"`
    Diagnostics  bool   `json:"diagnostics"`
    ChunkMaxSize *int   `json:"chunk_max_size,omitempty"`
}
```

### `NewProcessConfig(language string) ProcessConfig`

Create a ProcessConfig with all extraction options enabled (structure, imports, exports, comments, docstrings, symbols, diagnostics) and no chunking.

### `ProcessResult`

```go
type ProcessResult struct {
    Metadata FileMetadata `json:"metadata"`
    Chunks   []CodeChunk  `json:"chunks"`
}
```

### `FileMetadata`

```go
type FileMetadata struct {
    Language    string          `json:"language"`
    Metrics     FileMetrics     `json:"metrics"`
    Structure   []StructureItem `json:"structure,omitempty"`
    Imports     []ImportInfo    `json:"imports,omitempty"`
    Exports     []ExportInfo    `json:"exports,omitempty"`
    Comments    []CommentInfo   `json:"comments,omitempty"`
    Docstrings  []DocstringInfo `json:"docstrings,omitempty"`
    Symbols     []SymbolInfo    `json:"symbols,omitempty"`
    Diagnostics []Diagnostic    `json:"diagnostics,omitempty"`
}
```

### `FileMetrics`

```go
type FileMetrics struct {
    TotalLines   int `json:"total_lines"`
    CodeLines    int `json:"code_lines"`
    CommentLines int `json:"comment_lines"`
    BlankLines   int `json:"blank_lines"`
    TotalBytes   int `json:"total_bytes"`
    NodeCount    int `json:"node_count"`
    ErrorCount   int `json:"error_count"`
    MaxDepth     int `json:"max_depth"`
}
```

### `StructureItem`

```go
type StructureItem struct {
    Kind       string          `json:"kind"`
    Name       *string         `json:"name,omitempty"`
    Visibility *string         `json:"visibility,omitempty"`
    Span       Span            `json:"span"`
    Children   []StructureItem `json:"children,omitempty"`
    Decorators []string        `json:"decorators,omitempty"`
    DocComment *string         `json:"doc_comment,omitempty"`
    Signature  *string         `json:"signature,omitempty"`
    BodySpan   *Span           `json:"body_span,omitempty"`
}
```

### `ImportInfo`

```go
type ImportInfo struct {
    Source     string   `json:"source"`
    Items      []string `json:"items,omitempty"`
    Alias      *string  `json:"alias,omitempty"`
    IsWildcard bool     `json:"is_wildcard"`
    Span       Span     `json:"span"`
}
```

### `ExportInfo`

```go
type ExportInfo struct {
    Name string `json:"name"`
    Kind string `json:"kind"`
    Span Span   `json:"span"`
}
```

### `CommentInfo`

```go
type CommentInfo struct {
    Text           string  `json:"text"`
    Kind           string  `json:"kind"`
    Span           Span    `json:"span"`
    AssociatedNode *string `json:"associated_node,omitempty"`
}
```

### `DocstringInfo`

```go
type DocstringInfo struct {
    Text           string       `json:"text"`
    Format         string       `json:"format"`
    Span           Span         `json:"span"`
    AssociatedItem *string      `json:"associated_item,omitempty"`
    ParsedSections []DocSection `json:"parsed_sections,omitempty"`
}
```

### `SymbolInfo`

```go
type SymbolInfo struct {
    Name           string  `json:"name"`
    Kind           string  `json:"kind"`
    Span           Span    `json:"span"`
    TypeAnnotation *string `json:"type_annotation,omitempty"`
    Doc            *string `json:"doc,omitempty"`
}
```

### `Diagnostic`

```go
type Diagnostic struct {
    Message  string `json:"message"`
    Severity string `json:"severity"`
    Span     Span   `json:"span"`
}
```

### `CodeChunk`

```go
type CodeChunk struct {
    Content   string    `json:"content"`
    StartByte int       `json:"start_byte"`
    EndByte   int       `json:"end_byte"`
    StartLine int       `json:"start_line"`
    EndLine   int       `json:"end_line"`
    Metadata  ChunkInfo `json:"metadata"`
}
```

### `Span`

```go
type Span struct {
    StartByte   int `json:"start_byte"`
    EndByte     int `json:"end_byte"`
    StartLine   int `json:"start_line"`
    StartColumn int `json:"start_column"`
    EndLine     int `json:"end_line"`
    EndColumn   int `json:"end_column"`
}
```

### `NodeInfo`

```go
type NodeInfo struct {
    Kind            string `json:"kind"`
    IsNamed         bool   `json:"is_named"`
    StartByte       int    `json:"start_byte"`
    EndByte         int    `json:"end_byte"`
    StartRow        int    `json:"start_row"`
    StartColumn     int    `json:"start_column"`
    EndRow          int    `json:"end_row"`
    EndColumn       int    `json:"end_column"`
    NamedChildCount int    `json:"named_child_count"`
    IsError         bool   `json:"is_error"`
    IsMissing       bool   `json:"is_missing"`
}
```

## Extraction Queries

Extraction queries are not yet available in the Go binding. See the [Extraction Queries guide](../guides/extraction.md) for usage in other languages.

## Concurrency

The `Registry` type is safe for concurrent use from multiple goroutines. All exported methods acquire the appropriate lock before accessing the underlying C registry. Create separate Registry instances if you need independent registries:

```go
// Safe: methods on the same registry are synchronized
reg, _ := tspack.NewRegistry()
defer reg.Close()

var wg sync.WaitGroup
for i := 0; i < 10; i++ {
    wg.Add(1)
    go func() {
        defer wg.Done()
        tree, _ := reg.ParseString("python", "x = 1")
        defer tree.Close()
    }()
}
wg.Wait()
```
