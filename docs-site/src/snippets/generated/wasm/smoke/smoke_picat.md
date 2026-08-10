```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("main => true.\n", { language: "picat" });
}

void main();

```
