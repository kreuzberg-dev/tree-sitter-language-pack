---
id: fixture_node_smoke_verilog
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("module main; endmodule", { language: "verilog" });
}

void main();

```
