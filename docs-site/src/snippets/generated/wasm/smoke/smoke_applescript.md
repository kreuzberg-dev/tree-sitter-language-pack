```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("set x to 1\n", { language: "applescript" });
}

void main();

```
