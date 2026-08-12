---
id: fixture_node_smoke_sosl
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("FIND {test}\n", { language: "sosl" });
}

void main();

```
