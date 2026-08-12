---
id: fixture_node_smoke_koka
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("fun main()\n  1\n", { language: "koka" });
}

void main();

```
