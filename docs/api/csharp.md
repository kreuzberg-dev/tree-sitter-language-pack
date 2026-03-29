---
description: "C# / .NET API reference for tree-sitter-language-pack"
---

# C# / .NET API Reference

Requires .NET 10+.

Namespace: `TreeSitterLanguagePack`

## Installation

Add to `.csproj`:

```xml
<PackageReference Include="TreeSitterLanguagePack" Version="1.3.0" />
```

Or via dotnet CLI:

```bash
dotnet add package TreeSitterLanguagePack
```

## Quick Start

```csharp
using TreeSitterLanguagePack;

// List available languages
string[] langs = TsPackClient.AvailableLanguages();
Console.WriteLine($"{langs.Length} languages available");

// Parse source code
using var tree = TsPackClient.Parse("python", "def hello(): pass");
Console.WriteLine(tree.RootNodeType());           // "module"
Console.WriteLine(tree.ContainsNodeType("function_definition")); // True

// Extract code intelligence
var config = new ProcessConfig { Language = "python" };
var result = TsPackClient.Process("def hello(): pass", config);
Console.WriteLine($"Functions: {result.Structure.Count}");
```

## TsPackClient

Static class providing the high-level API. Manages a shared registry instance internally.

### Language Discovery

#### `AvailableLanguages(): string[]`

Get the list of all available language names.

**Returns:** string[]

**Example:**

```csharp
string[] languages = TsPackClient.AvailableLanguages();
foreach (var lang in languages)
{
    Console.WriteLine(lang);
}
```

#### `HasLanguage(string name): bool`

Check whether a language with the given name is available.

**Parameters:**

- `name` (string): Language name

**Returns:** bool

#### `LanguageCount(): int`

Get the number of available languages.

**Returns:** int

#### `GetLanguage(string name): IntPtr`

Get a raw TSLanguage pointer for the given language name.

**Parameters:**

- `name` (string): Language name

**Returns:** IntPtr

**Throws:**

- `TsPackException`: If the language is not available

### Language Detection

#### `DetectLanguage(string path): string?`

Detect language name from a file path or extension. Returns null if not recognized.

**Parameters:**

- `path` (string): File path

**Returns:** string?

**Example:**

```csharp
string? lang = TsPackClient.DetectLanguage("src/main.rs");
// lang == "rust"
```

#### `DetectLanguageFromContent(string content): string?`

Detect language name from file content using shebang-based detection. Returns null if no shebang is recognized.

**Parameters:**

- `content` (string): File content

**Returns:** string?

#### `DetectLanguageFromExtension(string ext): string?`

Detect language name from a bare file extension (without the leading dot). Returns null if not recognized.

**Parameters:**

- `ext` (string): File extension (e.g., `"rs"`, `"py"`)

**Returns:** string?

**Example:**

```csharp
string? lang = TsPackClient.DetectLanguageFromExtension("rs");
// lang == "rust"
```

#### `DetectLanguageFromPath(string path): string?`

Detect language name from a file path. Returns null if not recognized.

**Parameters:**

- `path` (string): File path

**Returns:** string?

**Example:**

```csharp
string? lang = TsPackClient.DetectLanguageFromPath("/home/user/project/main.py");
// lang == "python"
```

#### `ExtensionAmbiguity(string ext): ExtensionAmbiguityResult?`

Return ambiguity information for the given file extension. Returns null if the extension is not ambiguous.

**Parameters:**

- `ext` (string): File extension

**Returns:** ExtensionAmbiguityResult?

### Bundled Queries

#### `GetHighlightsQuery(string language): string?`

Return the bundled highlights query for the given language. Returns null if not available.

#### `GetInjectionsQuery(string language): string?`

Return the bundled injections query for the given language. Returns null if not available.

#### `GetLocalsQuery(string language): string?`

Return the bundled locals query for the given language. Returns null if not available.

### Parsing

#### `Parse(string languageName, string source): ParseTree`

Parse source code with the given language and return a `ParseTree` handle. The caller must dispose the returned `ParseTree`.

**Parameters:**

- `languageName` (string): Language name
- `source` (string): Source code

**Returns:** ParseTree

**Throws:**

- `TsPackException`: If parsing fails

**Example:**

```csharp
using var tree = TsPackClient.Parse("python", "def foo(): pass");
Console.WriteLine(tree.RootNodeType());  // "module"
Console.WriteLine(tree.RootChildCount()); // 1
```

### Code Intelligence

#### `Process(string source, ProcessConfig config): ProcessResult`

Process source code with the given configuration and return analysis results.

**Parameters:**

- `source` (string): Source code
- `config` (ProcessConfig): Configuration

**Returns:** ProcessResult

**Throws:**

- `TsPackException`: If processing fails

**Example:**

```csharp
var config = new ProcessConfig
{
    Language = "python",
    Comments = true,
    Docstrings = true,
};
var result = TsPackClient.Process("def hello(): pass", config);
Console.WriteLine($"Structure: {result.Structure.Count}");
Console.WriteLine($"Lines: {result.Metrics.TotalLines}");
```

### Download Management

#### `Init(string? configJson = null): void`

Initialize the language pack with configuration. `configJson` is a JSON string with optional fields: `"cache_dir"` (string), `"languages"` (array), `"groups"` (array).

**Parameters:**

- `configJson` (string?): JSON configuration (null for defaults)

**Throws:**

- `TsPackException`: If initialization fails

**Example:**

```csharp
TsPackClient.Init("""{"languages": ["python", "rust"]}""");
```

#### `Configure(string? configJson = null): void`

Configure the language pack cache directory without downloading.

**Parameters:**

- `configJson` (string?): JSON with optional `"cache_dir"` field

**Throws:**

- `TsPackException`: If configuration fails

#### `Download(params string[] languages): int`

Download specific languages to the cache. Returns the number of newly downloaded languages.

**Parameters:**

- `languages` (string[]): Language names to download

**Returns:** int

**Throws:**

- `TsPackException`: If download fails

**Example:**

```csharp
int count = TsPackClient.Download("python", "rust", "typescript");
Console.WriteLine($"Downloaded {count} new languages");
```

#### `DownloadAll(): int`

Download all available languages from the remote manifest. Returns the number of newly downloaded languages.

**Returns:** int

**Throws:**

- `TsPackException`: If download fails

#### `ManifestLanguages(): string[]`

Get all language names available in the remote manifest.

**Returns:** string[]

**Throws:**

- `TsPackException`: If the operation fails

#### `DownloadedLanguages(): string[]`

Get all languages that are already downloaded and cached locally. Returns an empty array if unavailable.

**Returns:** string[]

#### `CleanCache(): void`

Delete all cached parser shared libraries.

**Throws:**

- `TsPackException`: If the operation fails

#### `CacheDir(): string?`

Get the effective cache directory path.

**Returns:** string?

**Throws:**

- `TsPackException`: If the operation fails

## ParseTree

Opaque handle to a parsed syntax tree. Implements `IDisposable`. Must be disposed to free native memory.

### Methods

#### `RootNodeType(): string?`

Get the type name of the root node.

#### `RootChildCount(): uint`

Get the number of named children of the root node.

#### `ContainsNodeType(string nodeType): bool`

Check whether the tree contains a node with the given type name.

#### `HasErrorNodes(): bool`

Check whether the tree contains any ERROR or MISSING nodes.

#### `ToSexp(): string?`

Return the S-expression representation of the tree.

#### `ErrorCount(): int`

Return the count of ERROR and MISSING nodes in the tree.

#### `Dispose(): void`

Free the native tree handle.

**Example:**

```csharp
using var tree = TsPackClient.Parse("python", "def foo(): pass");
Console.WriteLine(tree.RootNodeType());                      // "module"
Console.WriteLine(tree.ContainsNodeType("function_definition")); // True
Console.WriteLine(tree.HasErrorNodes());                     // False
Console.WriteLine(tree.ToSexp());
```

## Types

### `ProcessConfig`

Configuration for `TsPackClient.Process()`. Serialized to JSON before passing to the FFI layer.

**Properties:**

```csharp
public sealed class ProcessConfig
{
    public required string Language { get; set; }
    public bool Structure { get; set; } = true;     // default: true
    public bool Imports { get; set; } = true;        // default: true
    public bool Exports { get; set; } = true;        // default: true
    public bool Comments { get; set; }               // default: false
    public bool Docstrings { get; set; }             // default: false
    public bool Symbols { get; set; }                // default: false
    public bool Diagnostics { get; set; }            // default: false
    public int? ChunkMaxSize { get; set; }           // default: null (no chunking)
}
```

**Example:**

```csharp
var config = new ProcessConfig
{
    Language = "python",
    Structure = true,
    Comments = true,
    ChunkMaxSize = 2000,
};
```

### `ProcessResult`

```csharp
public sealed class ProcessResult
{
    public string Language { get; set; }
    public FileMetrics Metrics { get; set; }
    public List<StructureItem> Structure { get; set; }
    public List<ImportInfo> Imports { get; set; }
    public List<ExportInfo> Exports { get; set; }
    public List<CommentInfo> Comments { get; set; }
    public List<DocstringInfo> Docstrings { get; set; }
    public List<SymbolInfo> Symbols { get; set; }
    public List<Diagnostic> Diagnostics { get; set; }
    public List<CodeChunk> Chunks { get; set; }
}
```

### `FileMetrics`

Properties: `TotalLines`, `CodeLines`, `CommentLines`, `BlankLines`, `TotalBytes`, `NodeCount`, `ErrorCount`, `MaxDepth` (all int).

### `Span`

Properties: `StartByte`, `EndByte`, `StartLine`, `StartColumn`, `EndLine`, `EndColumn` (all int).

### `StructureItem`

Properties: `Kind` (string), `Name` (string?), `Visibility` (string?), `Span` (Span), `Children` (List<StructureItem>), `Decorators` (List<string>), `DocComment` (string?), `Signature` (string?), `BodySpan` (Span?).

### `ImportInfo`

Properties: `Source` (string), `Items` (List<string>), `Alias` (string?), `IsWildcard` (bool), `Span` (Span).

### `ExportInfo`

Properties: `Name` (string), `Kind` (string), `Span` (Span).

### `CommentInfo`

Properties: `Text` (string), `Kind` (string), `Span` (Span), `AssociatedNode` (string?).

### `DocstringInfo`

Properties: `Text` (string), `Format` (string), `Span` (Span), `AssociatedItem` (string?), `ParsedSections` (List<DocSection>).

### `DocSection`

Properties: `Kind` (string), `Name` (string?), `Description` (string).

### `SymbolInfo`

Properties: `Name` (string), `Kind` (string), `Span` (Span), `TypeAnnotation` (string?), `Doc` (string?).

### `Diagnostic`

Properties: `Message` (string), `Severity` (string), `Span` (Span).

### `CodeChunk`

Properties: `Content` (string), `StartByte` (int), `EndByte` (int), `StartLine` (int), `EndLine` (int), `Metadata` (ChunkContext).

### `ChunkContext`

Properties: `Language` (string), `ChunkIndex` (int), `TotalChunks` (int), `NodeTypes` (List<string>), `ContextPath` (List<string>), `SymbolsDefined` (List<string>), `Comments` (List<CommentInfo>), `Docstrings` (List<DocstringInfo>), `HasErrorNodes` (bool).

### `ExtensionAmbiguityResult`

Properties: `Assigned` (string), `Alternatives` (string[]).

## Kind Constants

String constants are provided as static classes for type-safe comparisons:

- `StructureKind`: Function, Class, Method, Struct, Enum, Interface, Module, Namespace, Trait, TypeAlias, Constant, Field, Property, Other
- `ExportKind`: Function, Class, Constant, Type, Default, Namespace, Other
- `CommentKind`: Line, Block, Doc
- `DocstringFormat`: Markdown, ReStructuredText, GoogleStyle, NumpyStyle, Javadoc, XmlDoc, Plain, Other
- `SymbolKind`: Variable, Function, Class, Constant, Parameter, Field, Property, Type, Other
- `DiagnosticSeverity`: Error, Warning, Information, Hint

## Exception

### `TsPackException`

Thrown when a native FFI call fails. Extends `Exception`.

```csharp
public sealed class TsPackException : Exception
{
    public TsPackException(string message);
    public TsPackException(string message, Exception innerException);
}
```

## Extraction Queries

Extraction queries are not yet available in the C# binding. See the [Extraction Queries guide](../guides/extraction.md) for usage in other languages.

## Thread Safety

`TsPackClient` methods are safe to call from multiple threads. The shared registry instance is initialized with `LazyThreadSafetyMode.ExecutionAndPublication`.
