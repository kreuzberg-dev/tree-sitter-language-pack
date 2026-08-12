---
id: fixture_node_c_function_process
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("#include <stdio.h>\n\nint main() {\n    printf(\"hello\");\n    return 0;\n}\n", { language: "c" });
}

void main();

```
