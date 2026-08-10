```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("let () = print_endline \"hello\"", { language: "ocaml" });
}

void main();

```
