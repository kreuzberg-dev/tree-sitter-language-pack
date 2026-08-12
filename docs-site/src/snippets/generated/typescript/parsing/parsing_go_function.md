---
id: fixture_node_parsing_go_function
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("package main\nfunc main() {}", { language: "go" });
}

void main();

```
