---
id: fixture_wasm_smoke_task
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("todo item\n", { language: "task" });
}

void main();

```
