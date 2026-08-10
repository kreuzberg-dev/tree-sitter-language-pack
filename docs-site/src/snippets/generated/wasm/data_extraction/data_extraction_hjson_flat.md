```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("{\n  host: \"localhost\"\n  port: 8080\n}\n", { dataExtraction: true, language: "hjson" });
}

void main();

```
