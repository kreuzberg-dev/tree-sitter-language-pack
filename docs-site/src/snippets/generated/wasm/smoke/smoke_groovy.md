---
id: fixture_wasm_smoke_groovy
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("def x = 1", { language: "groovy" });
}

void main();

```
