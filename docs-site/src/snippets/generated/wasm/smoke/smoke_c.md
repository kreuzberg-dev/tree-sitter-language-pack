```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("int main() { return 0; }", { language: "c" });
}

void main();

```
