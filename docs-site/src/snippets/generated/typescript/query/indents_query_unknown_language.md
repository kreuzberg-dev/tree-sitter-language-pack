---
id: fixture_node_indents_query_unknown_language
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { getIndentsQuery } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = getIndentsQuery("nonexistent_xyz");
}

void main();

```
