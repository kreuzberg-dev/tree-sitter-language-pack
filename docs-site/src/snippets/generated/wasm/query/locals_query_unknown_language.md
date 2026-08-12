---
id: fixture_wasm_locals_query_unknown_language
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getLocalsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getLocalsQuery("nonexistent_xyz");
}

void main();

```
