---
id: fixture_node_smoke_typescript
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("const x: number = 42;", { language: "typescript" });
}

void main();

```
