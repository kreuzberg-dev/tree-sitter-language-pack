---
id: fixture_node_download_empty_list
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { download } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = download([]);
}

void main();

```
