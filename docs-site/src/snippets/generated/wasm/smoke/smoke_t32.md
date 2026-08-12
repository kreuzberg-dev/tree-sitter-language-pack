---
id: fixture_wasm_smoke_t32
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("PRINT 1\n", { language: "t32" });
}

void main();

```
