---
id: fixture_wasm_smoke_gomod
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("module example.com/hello\n\ngo 1.21", { language: "gomod" });
}

void main();

```
