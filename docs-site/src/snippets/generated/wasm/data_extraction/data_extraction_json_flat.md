```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("{\"host\": \"localhost\", \"port\": 8080}", { dataExtraction: true, language: "json" });
}

void main();

```
