---
id: fixture_node_injections_query_unknown_language
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { getInjectionsQuery } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = getInjectionsQuery("nonexistent_xyz");
}

void main();

```
