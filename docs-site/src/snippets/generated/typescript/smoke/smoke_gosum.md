---
id: fixture_node_smoke_gosum
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("example.com/pkg v1.0.0 h1:abc=", { language: "gosum" });
}

void main();

```
