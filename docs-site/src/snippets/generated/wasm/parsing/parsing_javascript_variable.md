---
id: fixture_wasm_parsing_javascript_variable
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("const x = 1;", { language: "javascript" });
}

void main();

```
