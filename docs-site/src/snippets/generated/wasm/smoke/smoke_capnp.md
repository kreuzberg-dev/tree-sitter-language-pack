---
id: fixture_wasm_smoke_capnp
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("@0xabcdef1234567890;", { language: "capnp" });
}

void main();

```
