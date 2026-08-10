```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("font_size 12\n", { language: "kitty" });
}

void main();

```
