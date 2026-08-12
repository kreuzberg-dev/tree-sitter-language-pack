---
id: fixture_wasm_smoke_hyprlang
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("general { border_size = 1 }", { language: "hyprlang" });
}

void main();

```
