---
id: fixture_node_smoke_scfg
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("key value\n", { language: "scfg" });
}

void main();

```
