```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("host = \"localhost\"\nport = 8080\n", { dataExtraction: true, language: "hocon" });
}

void main();

```
