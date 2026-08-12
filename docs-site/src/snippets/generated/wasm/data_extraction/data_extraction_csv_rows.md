---
id: fixture_wasm_data_extraction_csv_rows
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("a,b,c\n1,2,3\n", { dataExtraction: true, language: "csv" });
}

void main();

```
