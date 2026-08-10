```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("super + a\n\techo hi\n", { language: "sxhkdrc" });
}

void main();

```
