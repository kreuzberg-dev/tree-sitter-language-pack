---
id: fixture_node_smoke_luau
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("local x: number = 1", { language: "luau" });
}

void main();

```
