---
id: fixture_wasm_smoke_jjdescription
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("commit message\n", { language: "jjdescription" });
}

void main();

```
