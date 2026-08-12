---
id: fixture_wasm_smoke_racket
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("#lang racket\n(define x 1)", { language: "racket" });
}

void main();

```
