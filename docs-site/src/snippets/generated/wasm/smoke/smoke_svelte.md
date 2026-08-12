---
id: fixture_wasm_smoke_svelte
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<script>let x = 1;</script>", { language: "svelte" });
}

void main();

```
