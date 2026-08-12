---
id: fixture_node_error_process_empty_source
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("", { language: "python" });
}

void main();

```
