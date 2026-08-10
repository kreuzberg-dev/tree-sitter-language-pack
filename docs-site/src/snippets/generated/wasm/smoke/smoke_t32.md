```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("PRINT 1\n", { language: "t32" });
}

void main();

```
