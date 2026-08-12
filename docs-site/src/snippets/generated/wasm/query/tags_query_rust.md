---
id: fixture_wasm_tags_query_rust
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getTagsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getTagsQuery("rust");
}

void main();

```
