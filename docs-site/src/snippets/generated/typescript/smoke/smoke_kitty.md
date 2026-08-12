---
id: fixture_node_smoke_kitty
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("font_size 12\n", { language: "kitty" });
}

void main();

```
