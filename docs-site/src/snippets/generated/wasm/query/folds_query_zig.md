---
id: fixture_wasm_folds_query_zig
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getFoldsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getFoldsQuery("zig");
}

void main();

```
