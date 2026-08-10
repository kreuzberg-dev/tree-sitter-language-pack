```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("program test.aleo {\n}\n", { language: "leo" });
}

void main();

```
