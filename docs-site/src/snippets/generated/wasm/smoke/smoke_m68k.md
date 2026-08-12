---
id: fixture_wasm_smoke_m68k
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process(" move.l d0,d1\n", { language: "m68k" });
}

void main();

```
