```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("actor {\n}\n", { language: "motoko" });
}

void main();

```
