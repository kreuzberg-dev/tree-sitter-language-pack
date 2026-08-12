---
id: fixture_node_locals_query_unknown_language
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { getLocalsQuery } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = getLocalsQuery("nonexistent_xyz");
}

void main();

```
