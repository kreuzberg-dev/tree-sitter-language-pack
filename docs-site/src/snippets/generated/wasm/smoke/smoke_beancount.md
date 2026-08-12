---
id: fixture_wasm_smoke_beancount
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("2024-01-01 open Assets:Bank USD", { language: "beancount" });
}

void main();

```
