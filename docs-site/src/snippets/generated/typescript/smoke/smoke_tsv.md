---
id: fixture_node_smoke_tsv
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("a\tb\tc\n1\t2\t3", { language: "tsv" });
}

void main();

```
