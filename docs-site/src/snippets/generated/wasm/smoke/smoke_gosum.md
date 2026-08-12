---
id: fixture_wasm_smoke_gosum
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("example.com/pkg v1.0.0 h1:abc=", { language: "gosum" });
}

void main();

```
