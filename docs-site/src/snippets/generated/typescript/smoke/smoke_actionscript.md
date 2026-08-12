---
id: fixture_node_smoke_actionscript
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("var x:int = 1;", { language: "actionscript" });
}

void main();

```
