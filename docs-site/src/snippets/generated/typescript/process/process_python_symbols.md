---
id: fixture_node_process_python_symbols
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n", { language: "python", symbols: true });
}

void main();

```
