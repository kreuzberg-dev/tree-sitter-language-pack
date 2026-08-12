---
id: fixture_node_smoke_clarity
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("(define-public (hello) (ok true))", { language: "clarity" });
}

void main();

```
