---
id: fixture_node_smoke_cedar
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("permit(principal, action, resource);", { language: "cedar" });
}

void main();

```
