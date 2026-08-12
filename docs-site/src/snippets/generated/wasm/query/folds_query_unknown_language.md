---
id: fixture_wasm_folds_query_unknown_language
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getFoldsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getFoldsQuery("nonexistent_xyz");
}

void main();

```
