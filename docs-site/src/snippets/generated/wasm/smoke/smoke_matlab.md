---
id: fixture_wasm_smoke_matlab
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("function y = hello(x)\ny = x;\nend", { language: "matlab" });
}

void main();

```
