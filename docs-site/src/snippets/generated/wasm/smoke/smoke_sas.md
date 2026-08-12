---
id: fixture_wasm_smoke_sas
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("data _null_;\nrun;\n", { language: "sas" });
}

void main();

```
