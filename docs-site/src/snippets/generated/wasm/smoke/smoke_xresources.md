---
id: fixture_wasm_smoke_xresources
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("*.foreground: #ffffff\n", { language: "xresources" });
}

void main();

```
