---
id: fixture_wasm_smoke_yang
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("module m {\n}\n", { language: "yang" });
}

void main();

```
