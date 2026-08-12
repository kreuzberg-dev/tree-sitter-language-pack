---
id: fixture_node_smoke_sxhkdrc
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("super + a\n\techo hi\n", { language: "sxhkdrc" });
}

void main();

```
