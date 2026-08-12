---
id: fixture_node_smoke_capnp
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("@0xabcdef1234567890;", { language: "capnp" });
}

void main();

```
