---
id: fixture_node_parsing_typescript_function
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("function greet(name: string): string { return `hi ${name}`; }", { language: "typescript" });
}

void main();

```
