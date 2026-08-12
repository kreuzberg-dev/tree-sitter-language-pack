---
id: fixture_node_smoke_csv
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("a,b,c\n1,2,3", { language: "csv" });
}

void main();

```
