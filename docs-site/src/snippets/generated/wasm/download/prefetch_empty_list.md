---
id: fixture_wasm_prefetch_empty_list
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { prefetch } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = prefetch([]);
}

void main();

```
