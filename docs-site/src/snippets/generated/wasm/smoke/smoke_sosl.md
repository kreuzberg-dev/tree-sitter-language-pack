```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("FIND {test}\n", { language: "sosl" });
}

void main();

```
