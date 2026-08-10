```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("server:\n  host: localhost\n  port: 8080\n", { dataExtraction: true, language: "yaml" });
}

void main();

```
