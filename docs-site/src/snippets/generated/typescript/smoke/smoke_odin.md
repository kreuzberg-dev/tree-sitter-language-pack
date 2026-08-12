---
id: fixture_node_smoke_odin
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("package main", { language: "odin" });
}

void main();

```
