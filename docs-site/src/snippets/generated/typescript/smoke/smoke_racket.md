---
id: fixture_node_smoke_racket
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("#lang racket\n(define x 1)", { language: "racket" });
}

void main();

```
