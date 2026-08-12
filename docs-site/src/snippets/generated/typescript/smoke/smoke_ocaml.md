---
id: fixture_node_smoke_ocaml
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("let () = print_endline \"hello\"", { language: "ocaml" });
}

void main();

```
