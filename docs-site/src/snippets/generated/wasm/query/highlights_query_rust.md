---
id: fixture_wasm_highlights_query_rust
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getHighlightsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getHighlightsQuery("rust");
}

void main();

```
