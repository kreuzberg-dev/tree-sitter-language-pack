---
description: "WebAssembly API reference for tree-sitter-language-pack"
---

# WebAssembly API Reference

## Installation

### npm / Node.js

```bash
npm install @kreuzberg/tree-sitter-language-pack-wasm
```text

### Browser (ES Module)

```html
<script type="module">
  import * as tsp from "https://cdn.jsdelivr.net/npm/@kreuzberg/tree-sitter-language-pack-wasm";
</script>
```text

## Quick Start

```javascript
import * as tsp from "@kreuzberg/tree-sitter-language-pack-wasm";

// Get a language (note: download functions are stubs in WASM)
const language = tsp.getLanguage("python");

// Parse source code
const tree = tsp.parseString("def hello(): pass", language);
console.log(tree.rootNode.sexp());

// Extract code intelligence
const config = new tsp.ProcessConfig("python").all();
const result = tsp.process("def hello(): pass", config);
console.log("Functions:", result.structure.length);
```text

## Browser Usage

### ES Module

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <title>Tree-Sitter Language Pack</title>
  </head>
  <body>
    <textarea id="code" placeholder="Enter code..."></textarea>
    <button id="analyze">Analyze</button>
    <pre id="output"></pre>

    <script type="module">
      import * as tsp from "https://cdn.jsdelivr.net/npm/@kreuzberg/tree-sitter-language-pack-wasm";

      document.getElementById("analyze").addEventListener("click", () => {
        const code = document.getElementById("code").value;
        const config = new tsp.ProcessConfig("python").all();
        const result = tsp.process(code, config);
        document.getElementById("output").textContent = JSON.stringify(result, null, 2);
      });
    </script>
  </body>
</html>
```text

### CommonJS (Node.js)

```javascript
const tsp = require("@kreuzberg/tree-sitter-language-pack-wasm");

const language = tsp.getLanguage("python");
const tree = tsp.parseString("x = 1", language);
console.log(tree.rootNode.type); // "module"
```text

## Language Discovery

### `getLanguage(name: string): Language`

Get a tree-sitter Language by name.

Resolves aliases (e.g., `"shell"` → `"bash"`). Does **not** download (use pre-loaded grammars).

**Parameters:**

- `name` (string): Language name or alias

**Returns:** Language - tree-sitter Language object

**Throws:** Error if language not available

**Example:**

```javascript
const language = tsp.getLanguage("python");
console.log(language.name);
```text

### `availableLanguages(): string[]`

List all available language names.

**Returns:** string[] - Sorted language names

**Example:**

```javascript
const langs = tsp.availableLanguages();
console.log(`Available: ${langs.length} languages`);
```text

### `hasLanguage(name: string): boolean`

Check if a language is available.

**Parameters:**

- `name` (string): Language name or alias

**Returns:** boolean - True if available

**Example:**

```javascript
if (tsp.hasLanguage("python")) {
  console.log("Python available");
}
```text

### `languageCount(): number`

Get total number of available languages.

**Returns:** number - Language count

**Example:**

```javascript
const count = tsp.languageCount();
console.log(`${count} languages available`);
```text

## Parsing

### `parseString(source: string, language: Language): Tree`

Parse source code into a syntax tree.

**Parameters:**

- `source` (string): Source code
- `language` (Language): tree-sitter Language object

**Returns:** Tree - Parsed syntax tree

**Throws:** Error if parsing fails

**Example:**

```javascript
const language = tsp.getLanguage("python");
const tree = tsp.parseString("def foo(): pass", language);
console.log(tree.rootNode.sexp());
```text

### `TreeNode`

Parsed syntax tree node.

**Properties:**

- `type` (string) - Node type
- `kind` (string) - Node kind
- `startPoint` (Point) - Start {row, column}
- `endPoint` (Point) - End {row, column}
- `childCount` (number) - Number of children
- `children` (TreeNode[]) - Child nodes
- `sexp` (string) - S-expression

**Methods:**

- `child(index: number): TreeNode | null` - Get child by index
- `text(source: string): string` - Get node text from source

**Example:**

```javascript
const tree = tsp.parseString("x = 1", language);
console.log(tree.rootNode.type); // "module"
console.log(tree.rootNode.childCount); // number of children

for (const child of tree.rootNode.children) {
  console.log(child.type);
}
```text

## Code Intelligence

### `process(source: string, config: ProcessConfig): ProcessResult`

Extract code intelligence from source code.

**Parameters:**

- `source` (string): Source code
- `config` (ProcessConfig): Configuration

**Returns:** ProcessResult - Analysis result

**Throws:** Error if analysis fails

**Example:**

```javascript
const config = new tsp.ProcessConfig("python")
  .structure()
  .importExports()
  .withChunks(1000, 200);

const result = tsp.process("def hello(): pass", config);
console.log("Functions:", result.structure.length);
console.log("Lines:", result.metrics.totalLines);
```text

## Types

### `ProcessConfig`

Configuration for code intelligence analysis.

**Constructor:**

```javascript
const config = new tsp.ProcessConfig("python");
```text

**Methods:**

- `structure(): ProcessConfig` - Enable structure extraction
- `importExports(): ProcessConfig` - Enable imports/exports
- `comments(): ProcessConfig` - Enable comments
- `docstrings(): ProcessConfig` - Enable docstrings
- `symbols(): ProcessConfig` - Enable symbols
- `metrics(): ProcessConfig` - Enable metrics
- `diagnostics(): ProcessConfig` - Enable diagnostics
- `withChunks(maxSize: number, overlap: number): ProcessConfig` - Configure chunking
- `all(): ProcessConfig` - Enable all features

**Example:**

```javascript
const config = new tsp.ProcessConfig("python")
  .structure()
  .importExports()
  .comments()
  .withChunks(2000, 400);
```text

### `ProcessResult`

Result from code intelligence analysis.

**Properties:**

```javascript
{
  language: string,
  metrics: FileMetrics,
  structure: StructureItem[],
  imports: ImportInfo[],
  exports: ExportInfo[],
  comments: CommentInfo[],
  docstrings: DocstringInfo[],
  symbols: SymbolInfo[],
  diagnostics: Diagnostic[],
  chunks: CodeChunk[],
  parseErrors: number
}
```text

**Example:**

```javascript
const result = tsp.process(source, config);

console.log(`Language: ${result.language}`);
console.log(`Functions: ${result.structure.length}`);
console.log(`Lines: ${result.metrics.totalLines}`);

for (const item of result.structure) {
  console.log(`  ${item.kind}: ${item.name}`);
}
```text

### `FileMetrics`

**Properties:**

- `totalLines` (number) - Total lines
- `codeLines` (number) - Code lines
- `commentLines` (number) - Comment lines
- `blankLines` (number) - Blank lines

### `StructureItem`

**Properties:**

- `kind` (string) - Item kind (function, class, etc.)
- `name` (string) - Item name
- `line` (number) - Start line
- `column` (number) - Start column

### `ImportInfo`

**Properties:**

- `module` (string) - Module name
- `specifiers` (string[]) - Imported names
- `line` (number) - Line number

### `ExportInfo`

**Properties:**

- `name` (string) - Export name
- `kind` (string) - Export kind
- `line` (number) - Line number

### `CodeChunk`

**Properties:**

- `content` (string) - Chunk text
- `startLine` (number) - Start line
- `endLine` (number) - End line

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
  import * as tsp from "https://cdn.jsdelivr.net/npm/@kreuzberg/tree-sitter-language-pack-wasm";

  window.parseCode = function() {
    const code = document.getElementById("code").value;
    const lang = document.getElementById("language").value;

    try {
      const language = tsp.getLanguage(lang);
      const tree = tsp.parseString(code, language);
      document.getElementById("output").textContent = tree.rootNode.sexp();
    } catch (error) {
      document.getElementById("output").textContent = `Error: ${error.message}`;
    }
  };
</script>
```text

### Node.js: Batch Processing

```javascript
import * as tsp from "@kreuzberg/tree-sitter-language-pack-wasm";
import fs from "fs";
import path from "path";

function analyzeFiles(dir, lang) {
  const language = tsp.getLanguage(lang);
  const config = new tsp.ProcessConfig(lang).all();

  const files = fs.readdirSync(dir)
    .filter(f => f.endsWith(`.${lang === 'python' ? 'py' : lang}`));

  for (const file of files) {
    const source = fs.readFileSync(path.join(dir, file), 'utf-8');
    const result = tsp.process(source, config);
    console.log(`${file}: ${result.structure.length} items`);
  }
}

analyzeFiles('./src', 'python');
```text

### Deno Integration

```javascript
import * as tsp from "https://cdn.jsdelivr.net/npm/@kreuzberg/tree-sitter-language-pack-wasm";

const code = await Deno.readTextFile("code.py");
const language = tsp.getLanguage("python");
const tree = tsp.parseString(code, language);
console.log(tree.rootNode.sexp());
```text

### Cloudflare Workers

```javascript
import * as tsp from "@kreuzberg/tree-sitter-language-pack-wasm";

export default {
  async fetch(request) {
    const { code, language } = await request.json();

    try {
      const lang = tsp.getLanguage(language);
      const tree = tsp.parseString(code, lang);

      return new Response(
        JSON.stringify({ sexp: tree.rootNode.sexp() }),
        { headers: { "Content-Type": "application/json" } }
      );
    } catch (error) {
      return new Response(
        JSON.stringify({ error: error.message }),
        { status: 400 }
      );
    }
  },
};
```text

### Worker Threads (for CPU-intensive parsing)

```javascript
// worker.js
import * as tsp from "@kreuzberg/tree-sitter-language-pack-wasm";

parentPort.on('message', (message) => {
  const { code, language } = message;
  const lang = tsp.getLanguage(language);
  const result = tsp.process(code, new tsp.ProcessConfig(language).all());
  parentPort.postMessage(result);
});

// main.js
import { Worker } from 'worker_threads';
import path from 'path';

const worker = new Worker(path.resolve('worker.js'));

worker.postMessage({
  code: 'def hello(): pass',
  language: 'python'
});

worker.on('message', (result) => {
  console.log('Functions:', result.structure.length);
});
```text

### React Component

```jsx
import React, { useState } from 'react';
import * as tsp from '@kreuzberg/tree-sitter-language-pack-wasm';

export function CodeAnalyzer() {
  const [code, setCode] = useState('');
  const [language, setLanguage] = useState('python');
  const [result, setResult] = useState(null);

  const handleAnalyze = () => {
    try {
      const config = new tsp.ProcessConfig(language).all();
      const analyzed = tsp.process(code, config);
      setResult(analyzed);
    } catch (error) {
      setResult({ error: error.message });
    }
  };

  return (
    <div>
      <textarea value={code} onChange={(e) => setCode(e.target.value)} />
      <select value={language} onChange={(e) => setLanguage(e.target.value)}>
        <option>python</option>
        <option>javascript</option>
        <option>rust</option>
      </select>
      <button onClick={handleAnalyze}>Analyze</button>
      {result && <pre>{JSON.stringify(result, null, 2)}</pre>}
    </div>
  );
}
```text

### Vue.js Component

```vue
<template>
  <div class="analyzer">
    <textarea v-model="code" placeholder="Enter code..."></textarea>
    <select v-model="language">
      <option value="python">Python</option>
      <option value="javascript">JavaScript</option>
      <option value="rust">Rust</option>
    </select>
    <button @click="analyze">Analyze</button>
    <pre v-if="result">{{ JSON.stringify(result, null, 2) }}</pre>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import * as tsp from '@kreuzberg/tree-sitter-language-pack-wasm';

const code = ref('');
const language = ref('python');
const result = ref(null);

function analyze() {
  try {
    const config = new tsp.ProcessConfig(language.value).all();
    result.value = tsp.process(code.value, config);
  } catch (error) {
    result.value = { error: error.message };
  }
}
</script>
```text

## Language Support

The WASM package includes a curated subset of **55 languages** optimized for browser and edge runtime use cases. This subset covers web development, popular backend languages, data formats, and common scripting languages.

Compiling all 173 supported languages into a single WASM binary exceeds the memory limits of standard build environments. Native bindings (Python, Node.js, Ruby, Go, Java, C#, Elixir, PHP, CLI) include **all 173 languages**.

**Included language categories:**

- **Web**: HTML, CSS, JavaScript, TypeScript, TSX, JSON, Vue, Svelte, Astro, GraphQL, SCSS, JSDoc, Twig, Prisma
- **Data formats**: TOML, XML, CSV, TSV, PSV, INI, Properties, RON, SQL, Protocol Buffers, KDL
- **JVM**: Java, Kotlin, Scala, Groovy, Clojure
- **Systems**: C, C++, Rust, Go, Zig, Swift
- **Scripting**: Python, Ruby, Lua, Bash, PHP, Elixir
- **Functional**: Haskell, OCaml, Elm, Gleam, Scheme
- **Other**: Dart, Markdown, Dockerfile, HCL, Git configs

Use `availableLanguages()` at runtime to get the exact list of supported languages.

## Limitations

WASM builds have some limitations:

1. **Language subset**: 55 of 173 languages are included (see above). For all languages, use native bindings.
2. **No download API**: Grammars are pre-bundled. For dynamic downloading, use platform-specific bindings (Python, Node.js, etc.)
3. **Single-threaded**: Run CPU-intensive parsing in Web Workers
4. **No file I/O**: Read files from memory or streams
5. **Module size**: ~5-10MB for bundled grammars

## Performance Tips

1. **Reuse language objects** - Get language once, use many times
2. **Use Web Workers** - Parse large files in background threads
3. **Batch processing** - Parse multiple files together
4. **Memory management** - Clear trees after processing if memory is tight

```javascript
// Good: Reuse language
const pythonLang = tsp.getLanguage('python');
for (const file of files) {
  const tree = tsp.parseString(file.content, pythonLang);
  // ...
}

// Avoid: Getting language repeatedly
for (const file of files) {
  const tree = tsp.parseString(file.content, tsp.getLanguage('python'));
}
```
