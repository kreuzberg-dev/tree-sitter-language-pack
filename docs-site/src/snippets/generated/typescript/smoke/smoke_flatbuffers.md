---
id: fixture_node_smoke_flatbuffers
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("table Foo {}\n", { language: "flatbuffers" });
}

void main();

```
