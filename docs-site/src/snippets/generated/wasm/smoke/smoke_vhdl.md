```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("entity main is end main;", { language: "vhdl" });
}

void main();

```
