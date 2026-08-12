---
id: fixture_node_smoke_purescript
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("module Main where", { language: "purescript" });
}

void main();

```
