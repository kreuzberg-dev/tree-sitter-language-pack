---
id: fixture_wasm_error_handling_empty_source
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("", { language: "javascript" });
}

void main();

```
