---
id: fixture_wasm_data_extraction_csv_single_row
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("x,y,z\n", { dataExtraction: true, language: "csv" });
}

void main();

```
