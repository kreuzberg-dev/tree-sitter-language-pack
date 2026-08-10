```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("G0 X0\n", { language: "gcode" });
}

void main();

```
