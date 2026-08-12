---
id: fixture_node_smoke_m68k
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process(" move.l d0,d1\n", { language: "m68k" });
}

void main();

```
