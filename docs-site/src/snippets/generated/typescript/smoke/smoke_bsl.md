---
id: fixture_node_smoke_bsl
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("Procedure Main() EndProcedure", { language: "bsl" });
}

void main();

```
