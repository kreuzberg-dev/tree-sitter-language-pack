```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("def hello():\n    pass\n", { language: "python" });
}

void main();

```
