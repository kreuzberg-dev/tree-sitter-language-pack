---
id: fixture_node_smoke_haskell
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("main = putStrLn \"hello\"", { language: "haskell" });
}

void main();

```
