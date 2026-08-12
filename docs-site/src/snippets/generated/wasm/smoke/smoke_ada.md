---
id: fixture_wasm_smoke_ada
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("procedure Main is begin null; end Main;", { language: "ada" });
}

void main();

```
