---
id: fixture_node_smoke_yul
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("object \"C\" {\n  code {\n  }\n}\n", { language: "yul" });
}

void main();

```
