---
id: fixture_node_smoke_applescript
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("set x to 1\n", { language: "applescript" });
}

void main();

```
