---
id: fixture_node_smoke_hyprlang
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("general { border_size = 1 }", { language: "hyprlang" });
}

void main();

```
