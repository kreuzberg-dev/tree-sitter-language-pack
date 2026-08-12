---
id: fixture_node_folds_query_unknown_language
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { getFoldsQuery } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = getFoldsQuery("nonexistent_xyz");
}

void main();

```
