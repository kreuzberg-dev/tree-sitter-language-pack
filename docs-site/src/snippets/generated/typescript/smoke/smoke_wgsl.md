---
id: fixture_node_smoke_wgsl
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }", { language: "wgsl" });
}

void main();

```
