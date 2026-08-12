---
id: fixture_node_smoke_groovy
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("def x = 1", { language: "groovy" });
}

void main();

```
