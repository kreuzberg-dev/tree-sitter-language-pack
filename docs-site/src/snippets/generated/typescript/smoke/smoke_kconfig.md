---
id: fixture_node_smoke_kconfig
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("config FOO\n\tbool \"Enable foo\"", { language: "kconfig" });
}

void main();

```
