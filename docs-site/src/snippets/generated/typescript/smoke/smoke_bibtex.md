---
id: fixture_node_smoke_bibtex
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("@article{key, title={A}}", { language: "bibtex" });
}

void main();

```
