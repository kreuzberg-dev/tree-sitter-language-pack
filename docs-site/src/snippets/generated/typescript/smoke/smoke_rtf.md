---
id: fixture_node_smoke_rtf
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("{\\rtf1 hello}", { language: "rtf" });
}

void main();

```
