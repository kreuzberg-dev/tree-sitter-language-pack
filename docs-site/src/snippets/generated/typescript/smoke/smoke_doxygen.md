---
id: fixture_node_smoke_doxygen
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("/** @brief A function */", { language: "doxygen" });
}

void main();

```
