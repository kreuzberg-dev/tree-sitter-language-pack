---
id: fixture_node_python_function_process
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("def greet(name):\n    return f'Hello, {name}!'\n", { language: "python" });
}

void main();

```
