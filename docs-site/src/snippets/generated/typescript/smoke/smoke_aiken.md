---
id: fixture_node_smoke_aiken
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("fn main() {\n  1\n}\n", { language: "aiken" });
}

void main();

```
