---
id: fixture_node_smoke_promela
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("init {\n}\n", { language: "promela" });
}

void main();

```
