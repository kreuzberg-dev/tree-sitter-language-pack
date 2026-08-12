---
id: fixture_node_parsing_javascript_variable
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("const x = 1;", { language: "javascript" });
}

void main();

```
