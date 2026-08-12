---
id: fixture_node_prefetch_empty_list
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { prefetch } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = prefetch([]);
}

void main();

```
