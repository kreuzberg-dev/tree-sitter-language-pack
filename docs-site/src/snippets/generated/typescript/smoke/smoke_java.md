---
id: fixture_node_smoke_java
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("class Main {}", { language: "java" });
}

void main();

```
