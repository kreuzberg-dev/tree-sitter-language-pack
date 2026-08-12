---
id: fixture_wasm_smoke_soql
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("SELECT Id FROM Account\n", { language: "soql" });
}

void main();

```
