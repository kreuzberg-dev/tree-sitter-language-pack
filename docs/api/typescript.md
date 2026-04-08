---
description: "TypeScript/Node.js API reference for tree-sitter-language-pack"
---

# TypeScript / Node.js API Reference

## Installation

=== "npm"

    ```bash
    npm install @kreuzberg/tree-sitter-language-pack
    ```

=== "pnpm"

    ```bash
    pnpm add @kreuzberg/tree-sitter-language-pack
    ```

=== "yarn"

    ```bash
    yarn add @kreuzberg/tree-sitter-language-pack
    ```

The package ships pre-built native binaries for macOS (arm64), Linux (x64, arm64), and Windows (x64). Node.js >= 16 is required.

## Quick Example

```typescript
import {
  availableLanguages,
  detectLanguage,
  parseString,
  treeRootNodeType,
  treeHasErrorNodes,
  process,
} from "@kreuzberg/tree-sitter-language-pack";

// List available languages
const languages = availableLanguages();
console.log(`${languages.length} languages available`);

// Detect language from a file path
const lang = detectLanguage("main.py"); // "python"

// Parse source code
const tree = parseString("python", "def hello(): pass");
console.log(treeRootNodeType(tree)); // "module"
console.log(treeHasErrorNodes(tree)); // false

// Extract code intelligence
const result = process("def hello(): pass", { language: "python" });
console.log(result.metrics.totalLines);
```

All functions in this package are **synchronous**. None return Promises.

## Language Discovery

### `availableLanguages(): string[]`

Returns an array of all available language names, sorted alphabetically.

```typescript
import { availableLanguages } from "@kreuzberg/tree-sitter-language-pack";

const langs = availableLanguages();
// ["al", "asm", "bash", "c", "cpp", "css", ...]
```

### `hasLanguage(name: string): boolean`

Checks whether a language with the given name is available.

```typescript
import { hasLanguage } from "@kreuzberg/tree-sitter-language-pack";

hasLanguage("python"); // true
hasLanguage("nonexistent"); // false
```

### `languageCount(): number`

Returns the total number of available languages.

```typescript
import { languageCount } from "@kreuzberg/tree-sitter-language-pack";

const count = languageCount(); // 305
```

### `detectLanguage(path: string): string | null`

Detects a language name from a file path or extension. Returns `null` if the extension is not recognized.

```typescript
import { detectLanguage } from "@kreuzberg/tree-sitter-language-pack";

detectLanguage("src/main.py"); // "python"
detectLanguage("index.tsx"); // "tsx"
detectLanguage("unknown.xyz"); // null
```

### `detectLanguageFromExtension(ext: string): string | null`

Detects a language name from a bare file extension (without the leading dot). Returns `null` if the extension is not recognized.

```typescript
import { detectLanguageFromExtension } from "@kreuzberg/tree-sitter-language-pack";

detectLanguageFromExtension("rs"); // "rust"
detectLanguageFromExtension("js"); // "javascript"
detectLanguageFromExtension("xyz"); // null
```

### `detectLanguageFromPath(path: string): string | null`

Detects a language name from a file path. Equivalent to `detectLanguage`. Returns `null` if the extension is not recognized.

```typescript
import { detectLanguageFromPath } from "@kreuzberg/tree-sitter-language-pack";

detectLanguageFromPath("/home/user/project/lib.rs"); // "rust"
```

### `detectLanguageFromContent(content: string): string | null`

Detects a language name from file content using shebang-based detection. Returns `null` if no recognized shebang is found.

```typescript
import { detectLanguageFromContent } from "@kreuzberg/tree-sitter-language-pack";

detectLanguageFromContent("#!/usr/bin/env python3\nprint('hi')"); // "python"
detectLanguageFromContent("no shebang here"); // null
```

### `extensionAmbiguity(ext: string): AmbiguityResult | null`

Returns ambiguity information for a file extension that maps to multiple languages. Returns `null` if the extension is unambiguous.

**Return type:**

```typescript
interface AmbiguityResult {
  assigned: string;
  alternatives: string[];
}
```

```typescript
import { extensionAmbiguity } from "@kreuzberg/tree-sitter-language-pack";

const result = extensionAmbiguity("h");
// { assigned: "c", alternatives: ["cpp", "objective-c"] }

extensionAmbiguity("rs"); // null (unambiguous)
```

## Queries

### `getHighlightsQuery(language: string): string | null`

Returns the bundled tree-sitter highlights query for the given language, or `null` if none is available.

```typescript
import { getHighlightsQuery } from "@kreuzberg/tree-sitter-language-pack";

const query = getHighlightsQuery("python");
if (query) {
  console.log(query); // S-expression query string
}
```

### `getInjectionsQuery(language: string): string | null`

Returns the bundled tree-sitter injections query for the given language, or `null` if none is available.

```typescript
import { getInjectionsQuery } from "@kreuzberg/tree-sitter-language-pack";

const query = getInjectionsQuery("html");
// Query for embedded languages (script tags, style blocks, etc.)
```

### `getLocalsQuery(language: string): string | null`

Returns the bundled tree-sitter locals query for the given language, or `null` if none is available.

```typescript
import { getLocalsQuery } from "@kreuzberg/tree-sitter-language-pack";

const query = getLocalsQuery("python");
// Query for local variable scoping
```

## Parsing

### `parseString(language: string, source: string): ExternalObject<Tree>`

Parses a source string with the named language and returns an opaque tree handle. The returned handle is not a full tree-sitter `Tree` object -- use the `tree*` inspection functions below to read from it.

**Throws** if the language is not found or parsing fails.

```typescript
import { parseString } from "@kreuzberg/tree-sitter-language-pack";

const tree = parseString("python", "x = 1 + 2");
```

Note the parameter order: `language` first, then `source`.

### `treeRootNodeType(tree: ExternalObject<Tree>): string`

Returns the type name of the root node of a parsed tree.

```typescript
import { parseString, treeRootNodeType } from "@kreuzberg/tree-sitter-language-pack";

const tree = parseString("python", "x = 1");
treeRootNodeType(tree); // "module"
```

### `treeRootChildCount(tree: ExternalObject<Tree>): number`

Returns the number of named children of the root node.

```typescript
import { parseString, treeRootChildCount } from "@kreuzberg/tree-sitter-language-pack";

const tree = parseString("python", "x = 1\ny = 2");
treeRootChildCount(tree); // 2
```

### `treeContainsNodeType(tree: ExternalObject<Tree>, nodeType: string): boolean`

Checks whether any node in the tree has the given type name. Searches the entire tree recursively.

```typescript
import { parseString, treeContainsNodeType } from "@kreuzberg/tree-sitter-language-pack";

const tree = parseString("python", "def hello(): pass");
treeContainsNodeType(tree, "function_definition"); // true
treeContainsNodeType(tree, "class_definition"); // false
```

### `treeHasErrorNodes(tree: ExternalObject<Tree>): boolean`

Checks whether the tree contains any ERROR or MISSING nodes, indicating syntax errors.

```typescript
import { parseString, treeHasErrorNodes } from "@kreuzberg/tree-sitter-language-pack";

const good = parseString("python", "x = 1");
treeHasErrorNodes(good); // false

const bad = parseString("python", "def (broken");
treeHasErrorNodes(bad); // true
```

## Processing

### `process(source: string, config: JsProcessConfig): ProcessResult`

Extracts code intelligence from source code. Accepts a plain configuration object (not a class instance). Both camelCase and snake_case keys are accepted in the config; the result always uses camelCase keys.

**Throws** if the language is not found or processing fails.

**Config type:**

```typescript
interface JsProcessConfig {
  language: string;
  structure?: boolean;   // default: true
  imports?: boolean;     // default: true
  exports?: boolean;     // default: true
  comments?: boolean;    // default: true
  docstrings?: boolean;  // default: true
  symbols?: boolean;     // default: true
  diagnostics?: boolean; // default: true
  chunkMaxSize?: number; // optional, in bytes
  extractions?: Record<string, PatternConfig>; // custom extraction patterns
}
```

**Result type:**

```typescript
interface ProcessResult {
  language: string;
  metrics: FileMetrics;
  structure: StructureItem[];
  imports: ImportInfo[];
  exports: ExportInfo[];
  comments: CommentInfo[];
  docstrings: DocstringInfo[];
  symbols: SymbolInfo[];
  diagnostics: Diagnostic[];
  chunks: CodeChunk[];
}
```

```typescript
import { process } from "@kreuzberg/tree-sitter-language-pack";

// All extraction features enabled by default
const result = process("def hello(): pass", { language: "python" });
console.log(result.language); // "python"
console.log(result.metrics.totalLines); // 1
console.log(result.structure); // [{ kind: "function", name: "hello", ... }]

// Selective extraction
const minimal = process("import os\nx = 1", {
  language: "python",
  structure: false,
  comments: false,
  docstrings: false,
  symbols: false,
  diagnostics: false,
});
console.log(minimal.imports); // [{ module: "os", ... }]

// With chunking
const chunked = process(largeSource, {
  language: "python",
  chunkMaxSize: 1000,
});
for (const chunk of chunked.chunks) {
  console.log(chunk.content.length, chunk.metadata.nodeTypes);
}
```

### Supporting Types

```typescript
interface FileMetrics {
  totalLines: number;
  totalBytes: number;
  blankLines: number;
  commentLines: number;
  codeLines: number;
  errorCount: number;
}

interface Span {
  startByte: number;
  endByte: number;
  startRow: number;
  startCol: number;
  endRow: number;
  endCol: number;
}

interface StructureItem {
  kind: string;
  name: string;
  span: Span;
  parent: string | null;
}

interface ImportInfo {
  module: string;
  names: string[];
  span: Span;
}

interface ExportInfo {
  name: string;
  kind: string;
  span: Span;
}

interface CommentInfo {
  text: string;
  kind: string;
  span: Span;
  associatedNode: string | null;
}

interface DocstringInfo {
  text: string;
  format: string;
  span: Span;
  associatedItem: string | null;
  sections: Array<Record<string, string>>;
}

interface SymbolInfo {
  name: string;
  kind: string;
  span: Span;
  typeAnnotation: string | null;
}

interface Diagnostic {
  message: string;
  severity: string;
  span: Span;
}

interface CodeChunk {
  content: string;
  startByte: number;
  endByte: number;
  metadata: ChunkContext;
}

interface ChunkContext {
  language: string;
  chunkIndex: number;
  totalChunks: number;
  startLine: number;
  endLine: number;
  nodeTypes: string[];
  symbolsDefined: string[];
  comments: string[];
  docstrings: string[];
  hasErrorNodes: boolean;
  contextPath: string[];
}
```

## Extraction Queries

### `extract(source: string, config: object): ExtractionResult`

Run user-defined tree-sitter queries against source code and return structured results. Parses the source, executes all named patterns, and returns matches with captured nodes, text, and child fields.

Accepts both camelCase and snake_case config keys. Returns camelCase keys in the result.

**Throws** if the language is not found, parsing fails, or a query pattern is invalid.

**Config type:**

```typescript
interface ExtractionConfig {
  language: string;
  patterns: Record<string, PatternConfig>;
}

interface PatternConfig {
  query: string;                       // tree-sitter S-expression query
  captureOutput?: "Text" | "Node" | "Full";  // default: "Full"
  childFields?: string[];              // child field names to extract (default: [])
  maxResults?: number;                 // max matches to return (default: unlimited)
  byteRange?: [number, number];        // restrict to byte range (default: entire file)
}
```

**Result type:**

```typescript
interface ExtractionResult {
  language: string;
  results: Record<string, PatternResult>;
}

interface PatternResult {
  matches: MatchResult[];
  totalCount: number;
}

interface MatchResult {
  patternIndex: number;
  captures: CaptureResult[];
}

interface CaptureResult {
  name: string;
  node: NodeInfo | null;
  text: string | null;
  childFields: Record<string, string | null>;
  startByte: number;
}
```

```typescript
import { extract } from "@kreuzberg/tree-sitter-language-pack";

const result = extract("def hello(): pass\ndef world(): pass", {
  language: "python",
  patterns: {
    functions: {
      query: "(function_definition name: (identifier) @fn_name) @fn_def",
      captureOutput: "Full",
      childFields: ["name", "parameters"],
    },
  },
});

for (const match of result.results.functions.matches) {
  for (const capture of match.captures) {
    if (capture.text) {
      console.log(`${capture.name}: ${capture.text}`);
    }
  }
}
```

### `validateExtraction(config: object): ValidationResult`

Validate extraction patterns without executing them. Checks that the language exists and all query patterns compile successfully.

Accepts the same config shape as `extract`.

**Throws** if the language cannot be loaded or the config is malformed.

**Result type:**

```typescript
interface ValidationResult {
  valid: boolean;
  patterns: Record<string, PatternValidation>;
}

interface PatternValidation {
  valid: boolean;
  captureNames: string[];
  patternCount: number;
  warnings: string[];
  errors: string[];
}
```

```typescript
import { validateExtraction } from "@kreuzberg/tree-sitter-language-pack";

const result = validateExtraction({
  language: "python",
  patterns: {
    functions: {
      query: "(function_definition name: (identifier) @fn_name)",
    },
  },
});

console.log(result.valid); // true
console.log(result.patterns.functions.captureNames); // ["fn_name"]
```

## Download Management

These functions manage downloading and caching of parser shared libraries for languages that are not compiled into the native binary.

### `init(config?: JsPackConfig): void`

Initializes the download system with configuration and pre-downloads all specified languages.

**Throws** if configuration or download fails.

**Config type:**

```typescript
interface JsPackConfig {
  cacheDir?: string;
  languages?: string[];
  groups?: string[];
}
```

```typescript
import { init } from "@kreuzberg/tree-sitter-language-pack";

// Download specific languages
init({
  languages: ["python", "rust", "typescript"],
});

// Download language groups
init({
  groups: ["web", "data"],
});

// Custom cache directory
init({
  cacheDir: "/opt/ts-pack-cache",
  languages: ["python"],
});
```

### `configure(config: JsPackConfig): void`

Sets the cache directory and other options without downloading anything.

**Throws** if configuration fails.

```typescript
import { configure } from "@kreuzberg/tree-sitter-language-pack";

configure({ cacheDir: "/data/ts-pack-cache" });
```

### `download(names: string[]): number`

Downloads specific languages by name. Returns the number of languages successfully downloaded.

**Throws** if download fails.

```typescript
import { download } from "@kreuzberg/tree-sitter-language-pack";

const count = download(["python", "rust", "typescript"]);
console.log(`Downloaded ${count} languages`);
```

### `downloadAll(): number`

Downloads all available languages from the remote manifest. Returns the number of languages successfully downloaded.

**Throws** if download fails.

```typescript
import { downloadAll } from "@kreuzberg/tree-sitter-language-pack";

const count = downloadAll();
console.log(`Downloaded ${count} languages`);
```

### `manifestLanguages(): string[]`

Returns all available language names from the remote manifest.

**Throws** if the manifest fetch fails.

```typescript
import { manifestLanguages } from "@kreuzberg/tree-sitter-language-pack";

const languages = manifestLanguages();
console.log(`${languages.length} languages available for download`);
```

### `downloadedLanguages(): string[]`

Returns the names of all languages that have been downloaded and cached locally. Does not perform network requests.

```typescript
import { downloadedLanguages } from "@kreuzberg/tree-sitter-language-pack";

const cached = downloadedLanguages();
console.log(`Cached: ${cached.join(", ")}`);
```

### `cleanCache(): void`

Deletes all cached parser shared library files.

**Throws** if cache deletion fails.

```typescript
import { cleanCache } from "@kreuzberg/tree-sitter-language-pack";

cleanCache();
```

### `cacheDir(): string`

Returns the absolute path to the effective cache directory.

**Throws** if the cache directory cannot be determined.

```typescript
import { cacheDir } from "@kreuzberg/tree-sitter-language-pack";

const dir = cacheDir();
console.log(`Cache location: ${dir}`);
```

## Low-Level

### `getLanguagePtr(name: string): number`

Returns the raw `TSLanguage` pointer as a number, for interop with `node-tree-sitter` or other native tree-sitter bindings.

**Throws** if the language is not found.

```typescript
import { getLanguagePtr } from "@kreuzberg/tree-sitter-language-pack";
import Parser from "tree-sitter";

const ptr = getLanguagePtr("python");
const language = new Parser.Language(ptr);
const parser = new Parser();
parser.setLanguage(language);
const tree = parser.parse("x = 1");
console.log(tree.rootNode.type); // "module"
```

## Error Handling

All functions that can fail throw standard JavaScript `Error` objects with descriptive messages. There are no custom exception classes -- use `try`/`catch` and inspect `error.message`.

```typescript
import { parseString, getLanguagePtr } from "@kreuzberg/tree-sitter-language-pack";

try {
  getLanguagePtr("nonexistent_language");
} catch (error) {
  console.error(error.message); // describes the missing language
}
```

## Usage Patterns

### Detect and Parse

```typescript
import {
  detectLanguage,
  parseString,
  treeRootNodeType,
  treeHasErrorNodes,
} from "@kreuzberg/tree-sitter-language-pack";
import { readFileSync } from "node:fs";

const filePath = "src/main.py";
const lang = detectLanguage(filePath);
if (lang) {
  const source = readFileSync(filePath, "utf-8");
  const tree = parseString(lang, source);
  console.log(`Root: ${treeRootNodeType(tree)}, errors: ${treeHasErrorNodes(tree)}`);
}
```

### Analyze Multiple Files

```typescript
import { detectLanguage, process } from "@kreuzberg/tree-sitter-language-pack";
import { readFileSync } from "node:fs";

const files = ["app.py", "lib.rs", "index.ts"];

for (const file of files) {
  const lang = detectLanguage(file);
  if (!lang) continue;

  const source = readFileSync(file, "utf-8");
  const result = process(source, { language: lang });
  console.log(`${file}: ${result.structure.length} items, ${result.imports.length} imports`);
}
```

### Syntax Highlighting Queries

```typescript
import { getHighlightsQuery, getInjectionsQuery } from "@kreuzberg/tree-sitter-language-pack";

const highlights = getHighlightsQuery("python");
const injections = getInjectionsQuery("html");

if (highlights) {
  // Use with tree-sitter query API for syntax highlighting
  console.log(`Highlights query: ${highlights.length} bytes`);
}
```
