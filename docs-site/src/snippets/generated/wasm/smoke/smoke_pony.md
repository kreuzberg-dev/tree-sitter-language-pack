---
id: fixture_wasm_smoke_pony
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("actor Main\n  new create(env: Env) => None", { language: "pony" });
}

void main();

```
