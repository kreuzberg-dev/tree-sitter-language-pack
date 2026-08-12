---
id: fixture_node_config_minimal_python
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("def hello():\n    pass\n", { language: "python" });
}

void main();

```
