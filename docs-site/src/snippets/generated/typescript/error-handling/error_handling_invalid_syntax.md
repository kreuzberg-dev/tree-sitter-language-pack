---
id: fixture_node_error_handling_invalid_syntax
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("function function function @@@ %%%", { language: "javascript" });
}

void main();

```
