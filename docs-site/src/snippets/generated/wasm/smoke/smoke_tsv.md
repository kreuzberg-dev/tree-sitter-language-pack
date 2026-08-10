```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("a\tb\tc\n1\t2\t3", { language: "tsv" });
}

void main();

```
