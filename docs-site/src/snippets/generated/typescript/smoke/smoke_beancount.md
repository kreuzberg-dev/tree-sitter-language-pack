---
id: fixture_node_smoke_beancount
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("2024-01-01 open Assets:Bank USD", { language: "beancount" });
}

void main();

```
