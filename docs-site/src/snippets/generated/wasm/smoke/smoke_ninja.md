```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("rule cc\n  command = cc $in -o $out", { language: "ninja" });
}

void main();

```
