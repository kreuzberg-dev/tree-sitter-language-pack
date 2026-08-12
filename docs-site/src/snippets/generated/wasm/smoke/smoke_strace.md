---
id: fixture_wasm_smoke_strace
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("open(\"/x\", O_RDONLY) = 3\n", { language: "strace" });
}

void main();

```
