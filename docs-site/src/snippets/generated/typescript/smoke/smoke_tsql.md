---
id: fixture_node_smoke_tsql
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("SELECT 1;\n", { language: "tsql" });
}

void main();

```
