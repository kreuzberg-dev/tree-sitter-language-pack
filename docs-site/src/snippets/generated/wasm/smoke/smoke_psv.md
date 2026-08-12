---
id: fixture_wasm_smoke_psv
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("a|b|c\n1|2|3", { language: "psv" });
}

void main();

```
