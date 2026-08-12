---
id: fixture_node_smoke_jsdoc
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("/** @param {string} name */", { language: "jsdoc" });
}

void main();

```
