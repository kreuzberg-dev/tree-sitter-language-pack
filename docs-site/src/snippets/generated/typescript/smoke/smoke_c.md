---
id: fixture_node_smoke_c
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("int main() { return 0; }", { language: "c" });
}

void main();

```
