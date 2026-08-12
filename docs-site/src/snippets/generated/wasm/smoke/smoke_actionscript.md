---
id: fixture_wasm_smoke_actionscript
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("var x:int = 1;", { language: "actionscript" });
}

void main();

```
