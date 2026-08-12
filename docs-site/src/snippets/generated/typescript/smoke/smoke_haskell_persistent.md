---
id: fixture_node_smoke_haskell_persistent
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("Person\n  name String\n", { language: "haskell_persistent" });
}

void main();

```
