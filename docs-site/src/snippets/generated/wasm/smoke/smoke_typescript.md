---
id: fixture_wasm_smoke_typescript
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("const x: number = 42;", { language: "typescript" });
}

void main();

```
