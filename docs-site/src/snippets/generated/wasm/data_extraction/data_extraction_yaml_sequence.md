```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("ports:\n  - 8080\n  - 8081\n", { dataExtraction: true, language: "yaml" });
}

void main();

```
