```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("SELECT Id FROM Account\n", { language: "soql" });
}

void main();

```
