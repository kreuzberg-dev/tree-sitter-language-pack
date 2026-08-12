---
id: fixture_wasm_smoke_commonlisp
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("(defun hello () (print \"hello\"))", { language: "commonlisp" });
}

void main();

```
