---
id: fixture_node_rust_function_process
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", { language: "rust" });
}

void main();

```
