---
id: fixture_node_smoke_ungrammar
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("Root = Item*\nItem = 'token'", { language: "ungrammar" });
}

void main();

```
