---
id: fixture_wasm_smoke_aiken
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("fn main() {\n  1\n}\n", { language: "aiken" });
}

void main();

```
