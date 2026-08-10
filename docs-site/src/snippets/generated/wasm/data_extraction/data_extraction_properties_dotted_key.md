```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("server.host=localhost\nserver.port=8080\n", { dataExtraction: true, language: "properties" });
}

void main();

```
