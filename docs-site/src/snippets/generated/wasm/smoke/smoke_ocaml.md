```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("let () = print_endline \"hello\"", { language: "ocaml" });
}

void main();

```
