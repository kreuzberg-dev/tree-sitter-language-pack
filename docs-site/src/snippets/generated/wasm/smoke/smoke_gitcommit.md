---
id: fixture_wasm_smoke_gitcommit
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("feat: add feature\n\nBody text", { language: "gitcommit" });
}

void main();

```
