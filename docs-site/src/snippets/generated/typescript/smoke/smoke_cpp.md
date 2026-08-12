---
id: fixture_node_smoke_cpp
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("int main() { return 0; }", { language: "cpp" });
}

void main();

```
