---
id: fixture_node_smoke_cypher
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("MATCH (n) RETURN n\n", { language: "cypher" });
}

void main();

```
