---
id: fixture_wasm_tags_query_unknown_language
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getTagsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getTagsQuery("nonexistent_xyz");
}

void main();

```
