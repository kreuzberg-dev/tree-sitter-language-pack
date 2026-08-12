---
id: fixture_node_smoke_llvm
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("define i32 @main() { ret i32 0 }", { language: "llvm" });
}

void main();

```
