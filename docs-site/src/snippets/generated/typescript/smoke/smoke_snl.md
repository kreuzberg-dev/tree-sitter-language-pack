---
id: fixture_node_smoke_snl
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("program test\n", { language: "snl" });
}

void main();

```
