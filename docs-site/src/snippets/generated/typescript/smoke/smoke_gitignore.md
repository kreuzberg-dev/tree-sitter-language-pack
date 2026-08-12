---
id: fixture_node_smoke_gitignore
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("*.o\n*.log", { language: "gitignore" });
}

void main();

```
