---
id: fixture_node_smoke_luap
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("[a-z]+", { language: "luap" });
}

void main();

```
