---
id: fixture_node_smoke_wgsl_bevy
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("x", { language: "wgsl_bevy" });
}

void main();

```
