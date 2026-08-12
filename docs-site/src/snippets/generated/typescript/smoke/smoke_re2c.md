---
id: fixture_node_smoke_re2c
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("/*!re2c\n  [a-z]+ { return; }\n*/", { language: "re2c" });
}

void main();

```
