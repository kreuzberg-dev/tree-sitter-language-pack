---
description: "WebAssembly API reference for tree-sitter-language-pack"
---

# WebAssembly API Reference

## Installation

### npm / Node.js

```bash
npm install @kreuzberg/tree-sitter-language-pack-wasm
```

### Browser (ES Module)

```html
<script type="module">
  import * as tsp from "https://cdn.jsdelivr.net/npm/@kreuzberg/tree-sitter-language-pack-wasm";
</script>
```

## Quick Start

```javascript
import * as tsp from "@kreuzberg/tree-sitter-language-pack-wasm";

// List available languages
const langs = tsp.availableLanguages();
console.log(`${langs.length} languages available`);

// Parse source code (returns a WasmTree handle)
const tree = tsp.parseString("python", "def hello(): pass");
console.log(tsp.treeRootNodeType(tree));         // "module"
console.log(tsp.treeHasErrorNodes(tree));         // false
console.log(tsp.treeContainsNodeType(tree, "function_definition")); // true

// Free the tree when done (also freed by GC)
tsp.freeTree(tree);

// Extract code intelligence (config is a JS object)
const result = tsp.process("def hello(): pass", { language: "python" });
console.log("Structure items:", result.structure.length);
```

## Language Discovery

### `availableLanguages(): string[]`

List all available language names.

**Returns:** Array of strings (as JsValue array)

**Example:**

```javascript
const langs = tsp.availableLanguages();
console.log(`Available: ${langs.length} languages`);
```

### `hasLanguage(name: string): boolean`

Check if a language is available.

**Parameters:**

- `name` (string): Language name

**Returns:** boolean

**Example:**

```javascript
if (tsp.hasLanguage("python")) {
  console.log("Python available");
}
```

### `languageCount(): number`

Get total number of available languages.

**Returns:** number (u32)

**Example:**

```javascript
console.log(`${tsp.languageCount()} languages available`);
```

### `detectLanguage(path: string): string | null`

Detect language name from a file path or extension.

**Parameters:**

- `path` (string): File path or extension

**Returns:** string or null

**Example:**

```javascript
const lang = tsp.detectLanguage("script.py"); // "python"
```

### `detectLanguageFromContent(content: string): string | null`

Detect language name from file content (shebang-based detection).

**Parameters:**

- `content` (string): File content

**Returns:** string or null

**Example:**

```javascript
const lang = tsp.detectLanguageFromContent("#!/usr/bin/env python3\nprint('hello')");
// "python"
```

### `extensionAmbiguity(ext: string): string | null`

Returns extension ambiguity information as a JSON string, or null.

When non-null, parses to an object with `assigned` (string) and `alternatives` (string[]) fields.

**Parameters:**

- `ext` (string): File extension (without dot)

**Returns:** string (JSON) or null

**Example:**

```javascript
const info = tsp.extensionAmbiguity("h");
if (info) {
  const data = JSON.parse(info);
  console.log("Assigned:", data.assigned);
}
```

### `getLanguagePtr(name: string): number`

Returns the raw `TSLanguage` pointer as a u32 integer for wasm32 interop.

**Parameters:**

- `name` (string): Language name

**Returns:** number (u32 pointer)

**Throws:** Error if the language is not found.

**Example:**

```javascript
const ptr = tsp.getLanguagePtr("python");
console.log("Language pointer:", ptr);
```

## Queries

### `getHighlightsQuery(language: string): string | null`

Returns the bundled highlights query for the given language, or null.

### `getInjectionsQuery(language: string): string | null`

Returns the bundled injections query for the given language, or null.

### `getLocalsQuery(language: string): string | null`

Returns the bundled locals query for the given language, or null.

**Example:**

```javascript
const highlights = tsp.getHighlightsQuery("python");
if (highlights) {
  console.log(`Highlights query: ${highlights.length} bytes`);
}
```

## Parsing

### `parseString(language: string, source: string): WasmTree`

Parse source code and return an opaque `WasmTree` handle. Pass this handle to the `tree*` inspection functions.

**Parameters:**

- `language` (string): Language name
- `source` (string): Source code to parse

**Returns:** WasmTree - opaque tree handle

**Throws:** Error if the language is not found or parsing fails.

**Example:**

```javascript
const tree = tsp.parseString("python", "def foo(): pass");
console.log(tsp.treeRootNodeType(tree)); // "module"
```

## Tree Inspection Functions

These functions accept a `WasmTree` handle returned by `parseString`.

### `treeRootNodeType(tree: WasmTree): string`

Get the type name of the root node.

**Example:**

```javascript
const tree = tsp.parseString("python", "x = 1");
tsp.treeRootNodeType(tree); // "module"
```

### `treeRootChildCount(tree: WasmTree): number`

Get the number of named children of the root node.

**Returns:** number (u32)

**Example:**

```javascript
const tree = tsp.parseString("python", "x = 1\ny = 2");
tsp.treeRootChildCount(tree); // 2
```

### `treeContainsNodeType(tree: WasmTree, nodeType: string): boolean`

Check whether any node in the tree has the given type name.

**Example:**

```javascript
const tree = tsp.parseString("python", "def hello(): pass");
tsp.treeContainsNodeType(tree, "function_definition"); // true
```

### `treeHasErrorNodes(tree: WasmTree): boolean`

Check whether the tree contains any ERROR or MISSING nodes.

**Example:**

```javascript
const tree = tsp.parseString("python", "def (broken @@@ !!!");
tsp.treeHasErrorNodes(tree); // true
```

### `freeTree(tree: WasmTree): void`

Free the tree handle. Called automatically by JS garbage collection, but can be called manually to release memory sooner.

**Example:**

```javascript
const tree = tsp.parseString("python", "x = 1");
// ... use tree ...
tsp.freeTree(tree);
```

## Code Intelligence

### `process(source: string, config: object): object`

Process source code and extract metadata as a JavaScript object.

The config is a plain JS object (not a JSON string). It is converted internally via `JSON.stringify`.

**Parameters:**

- `source` (string): Source code
- `config` (object): Configuration object. Must contain at least `language`. Optional fields:
  - `structure` (bool, default true): Extract structural items
  - `imports` (bool, default true): Extract import statements
  - `exports` (bool, default true): Extract export statements
  - `comments` (bool, default false): Extract comments
  - `docstrings` (bool, default false): Extract docstrings
  - `symbols` (bool, default false): Extract symbol definitions
  - `diagnostics` (bool, default false): Include parse diagnostics
  - `chunk_max_size` (number or null, default null): Maximum chunk size in bytes

**Returns:** object - Parsed result as a native JS object

**Throws:** Error on invalid config, unknown language, or processing failure.

**Example:**

```javascript
const result = tsp.process("def hello(): pass", {
  language: "python",
  structure: true,
  comments: true,
  chunk_max_size: 2000,
});

for (const item of result.structure) {
  console.log(`${item.kind}: ${item.name}`);
}
```

## Pattern Extraction

### `extract(source: string, config: object): object`

Run tree-sitter queries against source code and return structured extraction results as a JavaScript object. Unlike `process`, which uses predefined intelligence queries, `extract` lets you supply arbitrary tree-sitter query patterns.

The config is a plain JS object (not a JSON string). It is converted internally via `JSON.stringify`.

**Parameters:**

- `source` (string): Source code to extract from
- `config` (object): Configuration object. Fields:
  - `language` (string, required): Language name
  - `patterns` (object, required): Named patterns to run. Each key maps to an object with:
    - `query` (string, required): tree-sitter query in S-expression syntax
    - `capture_output` (string, default `"Full"`): What to capture -- `"Text"`, `"Node"`, or `"Full"`
    - `child_fields` (string[], default `[]`): Field names to extract from child nodes
    - `max_results` (number | null, default null): Maximum number of matches to return
    - `byte_range` ([number, number] | null, default null): Restrict matches to a byte range

**Returns:** object - Extraction results. The top-level object contains:

- `language` (string): The language used
- `results` (object): Keyed by pattern name, each value contains:
  - `matches` (array): Each match has `pattern_index` (number) and `captures` (array). Each capture has `name` (string), `text` (string | null), `node` (object | null), `child_fields` (object), and `start_byte` (number).
  - `total_count` (number): Total matches before `max_results` truncation

**Throws:** Error on invalid config, unknown language, or extraction failure.

**Example:**

```javascript
const result = tsp.extract("def hello(): pass\ndef world(): pass", {
  language: "python",
  patterns: {
    functions: {
      query: "(function_definition name: (identifier) @fn_name)",
      capture_output: "Text",
    },
  },
});

for (const match of result.results.functions.matches) {
  for (const capture of match.captures) {
    console.log(capture.text);
  }
}
// Output:
// hello
// world
```

### `validateExtraction(config: object): object`

Validate extraction patterns without running them against source code. Useful for checking query syntax before performing extraction.

The config is a plain JS object with the same shape as the config for `extract`.

**Parameters:**

- `config` (object): Configuration object (must include `language` and `patterns`)

**Returns:** object - Validation results. The top-level object contains:

- `valid` (boolean): Whether all patterns are valid
- `patterns` (object): Per-pattern validation, each with:
  - `valid` (boolean): Whether this pattern compiled successfully
  - `capture_names` (string[]): Capture names defined in the query
  - `pattern_count` (number): Number of patterns in the query
  - `warnings` (string[]): Non-fatal warnings
  - `errors` (string[]): Fatal errors (e.g., query syntax errors)

**Throws:** Error on invalid config or unknown language.

**Example:**

```javascript
const result = tsp.validateExtraction({
  language: "python",
  patterns: {
    functions: {
      query: "(function_definition name: (identifier) @fn_name)",
    },
  },
});

if (result.valid) {
  console.log("All patterns valid");
} else {
  for (const [name, info] of Object.entries(result.patterns)) {
    if (!info.valid) {
      info.errors.forEach((err) => console.error(`${name}: ${err}`));
    }
  }
}
```

## Download/Configure API (Not Supported in WASM)

The following functions exist for API parity but are stubs. WASM cannot perform network I/O or maintain a persistent cache. All grammars are pre-bundled at compile time.

| Function | Behavior |
|----------|----------|
| `init(config)` | Always throws: "init/download not supported in WASM" |
| `configure(config)` | Always throws: "configure not supported in WASM" |
| `download(languages)` | Always throws: "download not supported in WASM" |
| `downloadAll()` | Always throws: "downloadAll not supported in WASM" |
| `manifestLanguages()` | Always throws: "manifestLanguages not supported in WASM" |
| `downloadedLanguages()` | Returns empty array |
| `cleanCache()` | No-op, returns successfully |
| `cacheDir()` | Always throws: "cacheDir not supported in WASM" |

## Language Support

The WASM package includes a curated subset of languages optimized for browser and edge runtime use cases. Compiling all 305 supported languages into a single WASM binary exceeds the memory limits of standard build environments. Native bindings (Python, Node.js, Ruby, Go, Java, C#, Elixir, PHP) include all 305 languages.

Use `availableLanguages()` at runtime to get the exact list of included languages.

## Limitations

1. **Language subset**: Not all 305 languages are included. For the full set, use native bindings.
2. **No download API**: Grammars are pre-bundled. Download functions throw errors.
3. **Single-threaded**: Run CPU-intensive parsing in Web Workers.
4. **No file I/O**: Read files into memory before parsing.

## Usage Patterns

### Browser: Parse User Code

```html
<textarea id="code"></textarea>
<select id="language">
  <option value="python">Python</option>
  <option value="javascript">JavaScript</option>
  <option value="rust">Rust</option>
</select>
<button onclick="parseCode()">Parse</button>
<pre id="output"></pre>

<script type="module">
  import * as tsp from "@kreuzberg/tree-sitter-language-pack-wasm";

  window.parseCode = function() {
    const code = document.getElementById("code").value;
    const lang = document.getElementById("language").value;

    try {
      const tree = tsp.parseString(lang, code);
      document.getElementById("output").textContent = tsp.treeRootNodeType(tree);
      tsp.freeTree(tree);
    } catch (error) {
      document.getElementById("output").textContent = `Error: ${error.message}`;
    }
  };
</script>
```

### Node.js: Batch Processing

```javascript
import * as tsp from "@kreuzberg/tree-sitter-language-pack-wasm";
import fs from "node:fs";
import path from "node:path";

function analyzeFiles(dir, lang) {
  const files = fs.readdirSync(dir).filter(f => f.endsWith(".py"));

  for (const file of files) {
    const source = fs.readFileSync(path.join(dir, file), "utf-8");
    const result = tsp.process(source, { language: lang });
    console.log(`${file}: ${result.structure.length} items`);
  }
}

analyzeFiles("./src", "python");
```

### Web Worker

```javascript
// worker.js
import * as tsp from "@kreuzberg/tree-sitter-language-pack-wasm";

self.onmessage = (event) => {
  const { code, language } = event.data;
  const result = tsp.process(code, { language });
  self.postMessage(result);
};

// main.js
const worker = new Worker("worker.js", { type: "module" });

worker.postMessage({ code: "def hello(): pass", language: "python" });

worker.onmessage = (event) => {
  console.log("Structure items:", event.data.structure.length);
};
```

## Performance Tips

1. **Reuse results** - Parse once, inspect multiple times using tree inspection functions.
2. **Free trees** - Call `freeTree()` when done to release memory promptly.
3. **Use Web Workers** - Parse large files in background threads.
4. **Batch processing** - Process multiple files in sequence to avoid repeated module initialization.
