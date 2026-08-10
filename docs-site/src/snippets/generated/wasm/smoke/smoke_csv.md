```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("a,b,c\n1,2,3", { language: "csv" });
}

void main();

```
