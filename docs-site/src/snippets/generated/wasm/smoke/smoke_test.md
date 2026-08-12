---
id: fixture_wasm_smoke_test
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("===========\nTest\n===========\n---\n(node)", { language: "test" });
}

void main();

```
