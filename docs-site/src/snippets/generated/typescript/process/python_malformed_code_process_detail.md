---
id: fixture_node_python_malformed_code_process_detail
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("def broken(\n    return\nclass", { diagnostics: true, language: "python" });
}

void main();

```
