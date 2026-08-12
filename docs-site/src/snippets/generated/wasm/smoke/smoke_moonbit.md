---
id: fixture_wasm_smoke_moonbit
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("fn main {\n}\n", { language: "moonbit" });
}

void main();

```
