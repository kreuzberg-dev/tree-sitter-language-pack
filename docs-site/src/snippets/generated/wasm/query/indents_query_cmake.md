---
id: fixture_wasm_indents_query_cmake
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getIndentsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getIndentsQuery("cmake");
}

void main();

```
