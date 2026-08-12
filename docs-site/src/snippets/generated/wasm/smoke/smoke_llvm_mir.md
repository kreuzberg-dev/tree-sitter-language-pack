---
id: fixture_wasm_smoke_llvm_mir
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("---\nname: foo\n...\n", { language: "llvm_mir" });
}

void main();

```
