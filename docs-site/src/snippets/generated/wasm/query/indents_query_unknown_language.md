---
id: fixture_wasm_indents_query_unknown_language
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getIndentsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getIndentsQuery("nonexistent_xyz");
}

void main();

```
