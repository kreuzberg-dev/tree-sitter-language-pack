```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("_method object.hello\n_endmethod", { language: "magik" });
}

void main();

```
