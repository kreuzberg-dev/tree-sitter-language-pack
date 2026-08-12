---
id: fixture_node_smoke_pony
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("actor Main\n  new create(env: Env) => None", { language: "pony" });
}

void main();

```
