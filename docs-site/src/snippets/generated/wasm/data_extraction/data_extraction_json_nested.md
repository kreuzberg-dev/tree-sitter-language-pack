```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("{\"server\": {\"host\": \"x\", \"port\": 8080}}", { dataExtraction: true, language: "json" });
}

void main();

```
