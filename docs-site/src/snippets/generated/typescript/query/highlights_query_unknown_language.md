---
id: fixture_node_highlights_query_unknown_language
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { getHighlightsQuery } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = getHighlightsQuery("nonexistent_language_xyz");
}

void main();

```
