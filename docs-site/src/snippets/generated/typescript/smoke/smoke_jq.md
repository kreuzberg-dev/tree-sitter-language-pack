---
id: fixture_node_smoke_jq
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process(".[] | select(.key)", { language: "jq" });
}

void main();

```
