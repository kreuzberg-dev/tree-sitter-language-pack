---
id: fixture_wasm_data_extraction_ini_section
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("[database]\nhost=localhost\nport=5432\n", { dataExtraction: true, language: "ini" });
}

void main();

```
