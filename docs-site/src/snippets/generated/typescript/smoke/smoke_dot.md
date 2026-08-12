---
id: fixture_node_smoke_dot
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("digraph G { A -> B; }", { language: "dot" });
}

void main();

```
