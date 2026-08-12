---
id: fixture_node_smoke_gstlaunch
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("fakesrc ! fakesink", { language: "gstlaunch" });
}

void main();

```
