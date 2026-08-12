---
id: fixture_node_smoke_ocaml_interface
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("val x : int", { language: "ocaml_interface" });
}

void main();

```
