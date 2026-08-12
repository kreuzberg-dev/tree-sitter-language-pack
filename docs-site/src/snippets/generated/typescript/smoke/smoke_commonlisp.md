---
id: fixture_node_smoke_commonlisp
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("(defun hello () (print \"hello\"))", { language: "commonlisp" });
}

void main();

```
