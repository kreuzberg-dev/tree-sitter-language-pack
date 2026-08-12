---
id: fixture_node_smoke_ninja
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("rule cc\n  command = cc $in -o $out", { language: "ninja" });
}

void main();

```
