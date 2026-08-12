---
id: fixture_node_smoke_xquery
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("1\n", { language: "xquery" });
}

void main();

```
