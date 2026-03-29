---
description: "Java API reference for tree-sitter-language-pack"
---

# Java API Reference

Package: `io.github.treesitter.languagepack`

Requires JDK 25+ (Panama Foreign Function and Memory API). No JNI.

## Installation

The native library must be available at runtime. Set the `TSPACK_LIB_PATH` environment variable to the path of the `libts_pack_ffi` shared library, or place it on the system library path.

## Quick Start

```java
import io.github.treesitter.languagepack.*;

public class Main {
    public static void main(String[] args) {
        try (var registry = new TsPackRegistry()) {
            // List available languages
            var languages = registry.availableLanguages();
            System.out.printf("%d languages available%n", languages.size());

            // Parse source code
            try (var tree = registry.parseString("python", "def hello(): pass")) {
                System.out.println(tree.rootNodeType());  // "module"
                System.out.println(tree.rootChildCount()); // 1
                System.out.println(tree.containsNodeType("function_definition")); // true
            }

            // Extract code intelligence (returns JSON)
            String configJson = """
                {"language": "python", "structure": true, "imports": true}
                """;
            String resultJson = registry.process("def hello(): pass", configJson);
            System.out.println(resultJson);
        }
    }
}
```

## TsPackRegistry

Implements `AutoCloseable`. Not thread-safe; callers must provide their own synchronization for concurrent access.

### Constructor

#### `new TsPackRegistry()`

Create a new language registry by calling the native `ts_pack_registry_new()`.

**Throws:**

- `RuntimeException`: If the native registry could not be created

**Example:**

```java
try (var registry = new TsPackRegistry()) {
    // use registry
}
```

### Instance Methods

#### `close(): void`

Free the underlying native registry. Safe to call multiple times. After closing, all other instance methods throw `IllegalStateException`.

#### `getLanguage(String name): MemorySegment`

Return the raw `TSLanguage*` pointer for the given language name. The returned `MemorySegment` remains valid for the lifetime of the registry.

**Parameters:**

- `name` (String): Language name (e.g., `"java"`, `"python"`)

**Returns:** MemorySegment pointing to the native TSLanguage struct

**Throws:**

- `LanguageNotFoundException`: If the language is not found
- `IllegalStateException`: If the registry has been closed

**Example:**

```java
try (var registry = new TsPackRegistry()) {
    MemorySegment lang = registry.getLanguage("java");
    // pass to a tree-sitter Java wrapper
}
```

#### `languageCount(): int`

Return the number of available languages in the registry.

**Returns:** int (non-negative)

**Throws:**

- `IllegalStateException`: If the registry has been closed

#### `languageNameAt(int index): String`

Return the language name at the given index.

**Parameters:**

- `index` (int): Zero-based index, must be in `[0, languageCount())`

**Returns:** String (never null or empty)

**Throws:**

- `IndexOutOfBoundsException`: If index is out of range
- `IllegalStateException`: If the registry has been closed

#### `hasLanguage(String name): boolean`

Check whether the registry contains a language with the given name.

**Parameters:**

- `name` (String): Language name

**Returns:** boolean

**Throws:**

- `IllegalStateException`: If the registry has been closed

#### `availableLanguages(): List<String>`

Return an unmodifiable list of all available language names.

**Returns:** List<String> (never null)

**Throws:**

- `IllegalStateException`: If the registry has been closed

**Example:**

```java
try (var registry = new TsPackRegistry()) {
    List<String> languages = registry.availableLanguages();
    for (String lang : languages) {
        System.out.println(lang);
    }
}
```

#### `parseString(String language, String source): TsPackTree`

Parse source code using the named language and return a tree handle. The returned `TsPackTree` must be closed when no longer needed.

**Parameters:**

- `language` (String): Language name
- `source` (String): Source code to parse

**Returns:** TsPackTree

**Throws:**

- `LanguageNotFoundException`: If the language is not found
- `IllegalStateException`: If the registry has been closed
- `RuntimeException`: If parsing fails

**Example:**

```java
try (var registry = new TsPackRegistry()) {
    try (var tree = registry.parseString("python", "x = 1")) {
        System.out.println(tree.rootNodeType()); // "module"
    }
}
```

#### `process(String source, String configJson): String`

Process source code and extract file intelligence as a JSON string. The `configJson` parameter must contain at least a `"language"` field. Optional boolean fields: `"structure"`, `"imports"`, `"exports"`, `"comments"`, `"docstrings"`, `"symbols"`, `"diagnostics"`. Optional integer field: `"chunk_max_size"`.

**Parameters:**

- `source` (String): Source code to process
- `configJson` (String): JSON configuration string

**Returns:** String (JSON result)

**Throws:**

- `IllegalStateException`: If the registry has been closed
- `RuntimeException`: If processing fails

**Example:**

```java
try (var registry = new TsPackRegistry()) {
    String json = registry.process(
        "def hello(): pass",
        "{\"language\": \"python\", \"structure\": true}"
    );
    System.out.println(json);
}
```

### Static Methods

#### `clearError(): void`

Clear the last error on the current thread.

#### `init(String configJson): void`

Initialize the language pack with configuration. `configJson` is a JSON string with optional fields: `"cache_dir"` (string), `"languages"` (array), `"groups"` (array).

**Parameters:**

- `configJson` (String): JSON configuration (may be null or empty)

**Throws:**

- `RuntimeException`: If initialization fails

**Example:**

```java
TsPackRegistry.init("{\"languages\": [\"python\", \"rust\"]}");
```

#### `configure(String configJson): void`

Configure the language pack cache directory without downloading.

**Parameters:**

- `configJson` (String): JSON with optional `"cache_dir"` field (may be null or empty)

**Throws:**

- `RuntimeException`: If configuration fails

#### `download(List<String> languages): int`

Download specific languages to the cache. Returns the number of newly downloaded languages.

**Parameters:**

- `languages` (List<String>): Language names to download

**Returns:** int

**Throws:**

- `RuntimeException`: If the download fails

**Example:**

```java
int count = TsPackRegistry.download(List.of("python", "rust", "typescript"));
System.out.printf("Downloaded %d new languages%n", count);
```

#### `downloadAll(): int`

Download all available languages from the remote manifest. Returns the number of newly downloaded languages.

**Returns:** int

**Throws:**

- `RuntimeException`: If the download fails

#### `manifestLanguages(): List<String>`

Get all language names available in the remote manifest. Returns an unmodifiable list.

**Returns:** List<String>

**Throws:**

- `RuntimeException`: If the operation fails

#### `downloadedLanguages(): List<String>`

Get all languages that are already downloaded and cached locally. Returns an unmodifiable list, or an empty list if unavailable.

**Returns:** List<String>

#### `cleanCache(): void`

Delete all cached parser shared libraries.

**Throws:**

- `RuntimeException`: If the operation fails

#### `cacheDir(): String`

Get the effective cache directory path.

**Returns:** String

**Throws:**

- `RuntimeException`: If the operation fails

### Static Methods -- Language Detection

#### `detectLanguage(String path): String`

Detect language name from a file path or extension. Returns null if not recognized.

**Parameters:**

- `path` (String): File path or extension

**Returns:** String (or null)

**Example:**

```java
String lang = TsPackRegistry.detectLanguage("src/main.rs");
// lang == "rust"
```

#### `detectLanguageFromContent(String content): String`

Detect language name from file content using shebang-based detection. Returns null if no shebang is recognized.

**Parameters:**

- `content` (String): File content

**Returns:** String (or null)

**Example:**

```java
String lang = TsPackRegistry.detectLanguageFromContent("#!/usr/bin/env python3\nprint('hi')");
// lang == "python"
```

#### `detectLanguageFromExtension(String ext): String`

Detect language name from a bare file extension (without the leading dot). Returns null if not recognized.

**Parameters:**

- `ext` (String): File extension (e.g., `"rs"`, `"py"`)

**Returns:** String (or null)

**Example:**

```java
String lang = TsPackRegistry.detectLanguageFromExtension("rs");
// lang == "rust"
```

#### `detectLanguageFromPath(String path): String`

Detect language name from a file path. Returns null if not recognized.

**Parameters:**

- `path` (String): File path

**Returns:** String (or null)

**Example:**

```java
String lang = TsPackRegistry.detectLanguageFromPath("/home/user/project/main.py");
// lang == "python"
```

#### `extensionAmbiguity(String ext): String`

Get extension ambiguity information as a JSON string. Returns null if the extension is not ambiguous.

**Parameters:**

- `ext` (String): File extension (without dot)

**Returns:** String (JSON with `"assigned"` and `"alternatives"` fields, or null)

**Example:**

```java
String json = TsPackRegistry.extensionAmbiguity("h");
// json contains {"assigned":"c","alternatives":["cpp","objective-c"]}
```

### Static Methods -- Queries

#### `getHighlightsQuery(String language): String`

Get the bundled highlights query for the given language. Returns null if not available.

**Parameters:**

- `language` (String): Language name

**Returns:** String (or null)

#### `getInjectionsQuery(String language): String`

Get the bundled injections query for the given language. Returns null if not available.

**Parameters:**

- `language` (String): Language name

**Returns:** String (or null)

#### `getLocalsQuery(String language): String`

Get the bundled locals query for the given language. Returns null if not available.

**Parameters:**

- `language` (String): Language name

**Returns:** String (or null)

### Static Methods -- Extraction

#### `extract(String source, String configJson): String`

Run extraction queries against source code using tree-sitter query patterns. Returns results as a JSON string.

**Parameters:**

- `source` (String): Source code to query
- `configJson` (String): JSON configuration with `"language"` and `"patterns"` fields

**Returns:** String (JSON result)

**Throws:**

- `RuntimeException`: If extraction fails

**Example:**

```java
String config = """
    {"language":"python","patterns":{"fns":{"query":"(function_definition name: (identifier) @fn_name)","capture_output":{},"child_fields":[],"max_results":null,"byte_range":null}}}
    """;
String result = TsPackRegistry.extract("def hello(): pass", config);
System.out.println(result);
```

#### `validateExtraction(String configJson): String`

Validate extraction patterns without running them against source code. Useful for checking query syntax.

**Parameters:**

- `configJson` (String): JSON configuration with the same shape as for `extract`

**Returns:** String (JSON validation result)

**Throws:**

- `RuntimeException`: If validation fails

**Example:**

```java
String config = """
    {"language":"python","patterns":{"fns":{"query":"(function_definition name: (identifier) @fn_name)","capture_output":{},"child_fields":[],"max_results":null,"byte_range":null}}}
    """;
String result = TsPackRegistry.validateExtraction(config);
System.out.println(result);
```

## TsPackTree

Implements `AutoCloseable`. Not thread-safe.

### Methods

#### `close(): void`

Free the underlying native tree. Safe to call multiple times.

#### `rootNodeType(): String`

Return the type name of the root node (e.g., `"module"` for Python).

**Returns:** String

**Throws:**

- `IllegalStateException`: If the tree has been closed

#### `rootChildCount(): int`

Return the number of named children of the root node.

**Returns:** int (non-negative)

**Throws:**

- `IllegalStateException`: If the tree has been closed

#### `containsNodeType(String nodeType): boolean`

Check whether any node in the tree has the given type name.

**Parameters:**

- `nodeType` (String): Node type to search for

**Returns:** boolean

**Throws:**

- `IllegalStateException`: If the tree has been closed

#### `hasErrorNodes(): boolean`

Check whether the tree contains any ERROR or MISSING nodes.

**Returns:** boolean

**Throws:**

- `IllegalStateException`: If the tree has been closed

#### `toSexp(): String`

Return the S-expression representation of the tree.

**Returns:** String

**Throws:**

- `IllegalStateException`: If the tree has been closed

**Example:**

```java
try (var tree = registry.parseString("python", "x = 1")) {
    String sexp = tree.toSexp();
    System.out.println(sexp); // (module (expression_statement (assignment ...)))
}
```

#### `errorCount(): int`

Return the count of ERROR and MISSING nodes in the tree.

**Returns:** int (non-negative)

**Throws:**

- `IllegalStateException`: If the tree has been closed

**Example:**

```java
try (var tree = registry.parseString("python", "def (broken")) {
    System.out.println(tree.errorCount()); // >= 1
}
```

## Exceptions

### `LanguageNotFoundException`

Thrown when a requested language is not available. Extends `IllegalArgumentException`.

**Methods:**

- `getLanguageName(): String` - Return the name of the language that was not found

## Model Records

These record types are used to deserialize JSON results from `process()`.

### `ProcessResult`

```java
public record ProcessResult(
    FileMetadata metadata,
    List<CodeChunk> chunks
) {}
```

### `FileMetadata`

```java
public record FileMetadata(
    String language,
    FileMetrics metrics,
    List<StructureItem> structure,
    List<ImportInfo> imports,
    List<ExportInfo> exports,
    List<CommentInfo> comments,
    List<DocstringInfo> docstrings,
    List<SymbolInfo> symbols,
    List<Diagnostic> diagnostics
) {}
```

### `FileMetrics`

Fields: `totalLines`, `codeLines`, `commentLines`, `blankLines`, `totalBytes`, `nodeCount`, `errorCount`, `maxDepth` (all int).

### `StructureItem`

Fields: `kind` (String), `name` (String), `visibility` (String), `span` (Span), `children` (List<StructureItem>), `decorators` (List<String>), `docComment` (String), `signature` (String), `bodySpan` (Span).

### `ImportInfo`

Fields: `source` (String), `items` (List<String>), `alias` (String), `isWildcard` (boolean), `span` (Span).

### `ExportInfo`

Fields: `name` (String), `kind` (String), `span` (Span).

### `CommentInfo`

Fields: `text` (String), `kind` (String), `span` (Span), `associatedNode` (String).

### `DocstringInfo`

Fields: `text` (String), `format` (String), `span` (Span), `associatedItem` (String), `parsedSections` (List<DocSection>).

### `DocSection`

Fields: `kind` (String), `name` (String), `description` (String).

### `SymbolInfo`

Fields: `name` (String), `kind` (String), `span` (Span), `typeAnnotation` (String), `doc` (String).

### `Diagnostic`

Fields: `message` (String), `severity` (String), `span` (Span).

### `CodeChunk`

Fields: `content` (String), `startByte` (int), `endByte` (int), `startLine` (int), `endLine` (int), `metadata` (ChunkInfo).

### `ChunkInfo`

Fields: `language` (String), `chunkIndex` (int), `totalChunks` (int), `nodeTypes` (List<String>), `contextPath` (List<String>), `symbolsDefined` (List<String>), `comments` (List<CommentInfo>), `docstrings` (List<DocstringInfo>), `hasErrorNodes` (boolean).

### `Span`

Fields: `startByte` (int), `endByte` (int), `startLine` (int), `startColumn` (int), `endLine` (int), `endColumn` (int).

### `NodeInfo`

Fields: `kind` (String), `isNamed` (boolean), `startByte` (int), `endByte` (int), `startRow` (int), `startColumn` (int), `endRow` (int), `endColumn` (int), `namedChildCount` (int), `isError` (boolean), `isMissing` (boolean).

## Extraction Queries

Extraction queries are not yet available in the Java binding. See the [Extraction Queries guide](../guides/extraction.md) for usage in other languages.

## Thread Safety

`TsPackRegistry` instances are **not** thread-safe. If concurrent access is required, callers must provide their own synchronization. Static methods (download, init, configure, etc.) do not require a registry instance.
