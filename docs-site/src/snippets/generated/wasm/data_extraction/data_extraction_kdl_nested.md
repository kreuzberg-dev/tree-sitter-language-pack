---
id: fixture_wasm_data_extraction_kdl_nested
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("server {\n  host \"localhost\"\n  port 8080\n}\n", { dataExtraction: true, language: "kdl" });
}

void main();

```
