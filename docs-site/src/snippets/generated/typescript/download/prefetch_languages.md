---
id: fixture_node_prefetch_languages
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { prefetch } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = prefetch(["python"]);
}

void main();

```
