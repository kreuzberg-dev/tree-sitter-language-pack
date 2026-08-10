```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("const x = 1;", { language: "javascript" });
}

void main();

```
