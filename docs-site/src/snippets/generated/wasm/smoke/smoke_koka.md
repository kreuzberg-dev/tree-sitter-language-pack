```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("fun main()\n  1\n", { language: "koka" });
}

void main();

```
