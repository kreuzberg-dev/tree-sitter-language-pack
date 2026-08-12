---
id: fixture_node_smoke_strace
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("open(\"/x\", O_RDONLY) = 3\n", { language: "strace" });
}

void main();

```
