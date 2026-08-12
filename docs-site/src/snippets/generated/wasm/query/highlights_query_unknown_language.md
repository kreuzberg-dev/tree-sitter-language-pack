---
id: fixture_wasm_highlights_query_unknown_language
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getHighlightsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getHighlightsQuery("nonexistent_language_xyz");
}

void main();

```
