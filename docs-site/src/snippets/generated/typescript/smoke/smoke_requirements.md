---
id: fixture_node_smoke_requirements
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("flask>=2.0", { language: "requirements" });
}

void main();

```
