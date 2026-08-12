---
id: fixture_node_smoke_devicetree
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("/dts-v1/;\n/ { };", { language: "devicetree" });
}

void main();

```
