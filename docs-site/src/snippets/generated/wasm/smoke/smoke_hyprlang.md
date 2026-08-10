```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("general { border_size = 1 }", { language: "hyprlang" });
}

void main();

```
