```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("set editing-mode vi", { language: "readline" });
}

void main();

```
