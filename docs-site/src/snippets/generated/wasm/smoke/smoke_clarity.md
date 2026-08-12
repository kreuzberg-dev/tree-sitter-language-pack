---
id: fixture_wasm_smoke_clarity
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("(define-public (hello) (ok true))", { language: "clarity" });
}

void main();

```
