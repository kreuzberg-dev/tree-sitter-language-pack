---
id: fixture_node_smoke_ada
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("procedure Main is begin null; end Main;", { language: "ada" });
}

void main();

```
