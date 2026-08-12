---
id: fixture_node_smoke_yang
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("module m {\n}\n", { language: "yang" });
}

void main();

```
