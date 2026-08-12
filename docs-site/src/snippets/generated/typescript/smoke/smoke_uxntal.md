---
id: fixture_node_smoke_uxntal
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("|0100 LIT 01", { language: "uxntal" });
}

void main();

```
