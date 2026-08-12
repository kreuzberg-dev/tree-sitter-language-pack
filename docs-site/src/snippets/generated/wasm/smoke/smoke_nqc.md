---
id: fixture_wasm_smoke_nqc
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("task main() {}", { language: "nqc" });
}

void main();

```
