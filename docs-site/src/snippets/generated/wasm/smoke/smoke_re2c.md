---
id: fixture_wasm_smoke_re2c
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("/*!re2c\n  [a-z]+ { return; }\n*/", { language: "re2c" });
}

void main();

```
