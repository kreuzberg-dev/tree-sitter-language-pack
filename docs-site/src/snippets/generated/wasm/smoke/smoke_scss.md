---
id: fixture_wasm_smoke_scss
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("$color: red;\nbody { color: $color; }", { language: "scss" });
}

void main();

```
