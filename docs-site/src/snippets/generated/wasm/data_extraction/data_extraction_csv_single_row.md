```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("x,y,z\n", { dataExtraction: true, language: "csv" });
}

void main();

```
