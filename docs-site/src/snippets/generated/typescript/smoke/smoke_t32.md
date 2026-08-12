---
id: fixture_node_smoke_t32
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("PRINT 1\n", { language: "t32" });
}

void main();

```
