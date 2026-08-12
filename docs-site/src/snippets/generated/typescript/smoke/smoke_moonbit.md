---
id: fixture_node_smoke_moonbit
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("fn main {\n}\n", { language: "moonbit" });
}

void main();

```
