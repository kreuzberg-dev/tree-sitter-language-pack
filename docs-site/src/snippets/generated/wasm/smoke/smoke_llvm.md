---
id: fixture_wasm_smoke_llvm
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("define i32 @main() { ret i32 0 }", { language: "llvm" });
}

void main();

```
