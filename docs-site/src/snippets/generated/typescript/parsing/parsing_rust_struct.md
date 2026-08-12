---
id: fixture_node_parsing_rust_struct
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("struct Point { x: f64, y: f64 }", { language: "rust" });
}

void main();

```
