---
id: fixture_node_smoke_corn
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("{ key = \"value\" }", { language: "corn" });
}

void main();

```
