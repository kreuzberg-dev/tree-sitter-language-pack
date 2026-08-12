---
id: fixture_wasm_smoke_wdl
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("version 1.0\n", { language: "wdl" });
}

void main();

```
