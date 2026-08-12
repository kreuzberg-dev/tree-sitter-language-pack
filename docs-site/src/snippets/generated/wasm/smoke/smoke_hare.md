---
id: fixture_wasm_smoke_hare
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("export fn main() void = void;", { language: "hare" });
}

void main();

```
