---
id: fixture_wasm_data_extraction_json_array
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("[1, 2, 3]", { dataExtraction: true, language: "json" });
}

void main();

```
