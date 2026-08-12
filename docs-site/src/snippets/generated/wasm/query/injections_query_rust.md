---
id: fixture_wasm_injections_query_rust
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getInjectionsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getInjectionsQuery("rust");
}

void main();

```
