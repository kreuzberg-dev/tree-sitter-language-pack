---
id: fixture_wasm_smoke_po
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("msgid \"hello\"\nmsgstr \"world\"", { language: "po" });
}

void main();

```
