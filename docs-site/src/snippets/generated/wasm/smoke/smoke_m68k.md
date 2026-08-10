```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process(" move.l d0,d1\n", { language: "m68k" });
}

void main();

```
