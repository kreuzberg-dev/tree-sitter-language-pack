---
id: fixture_wasm_smoke_ninja
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("rule cc\n  command = cc $in -o $out", { language: "ninja" });
}

void main();

```
