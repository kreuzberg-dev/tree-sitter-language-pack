---
id: fixture_wasm_data_extraction_json_empty_object
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("{}", { dataExtraction: true, language: "json" });
}

void main();

```
