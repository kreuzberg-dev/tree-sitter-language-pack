---
id: fixture_wasm_injections_query_unknown_language
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getInjectionsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getInjectionsQuery("nonexistent_xyz");
}

void main();

```
