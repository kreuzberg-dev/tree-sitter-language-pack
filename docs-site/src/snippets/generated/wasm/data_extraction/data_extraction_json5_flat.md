---
id: fixture_wasm_data_extraction_json5_flat
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("{\n  host: \"localhost\",\n  port: 8080,\n}\n", { dataExtraction: true, language: "json5" });
}

void main();

```
