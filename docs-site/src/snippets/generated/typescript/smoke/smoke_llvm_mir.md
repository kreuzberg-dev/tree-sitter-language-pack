---
id: fixture_node_smoke_llvm_mir
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("---\nname: foo\n...\n", { language: "llvm_mir" });
}

void main();

```
