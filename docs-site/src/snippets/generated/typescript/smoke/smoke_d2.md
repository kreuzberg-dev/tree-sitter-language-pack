---
id: fixture_node_smoke_d2
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("a -> b\n", { language: "d2" });
}

void main();

```
