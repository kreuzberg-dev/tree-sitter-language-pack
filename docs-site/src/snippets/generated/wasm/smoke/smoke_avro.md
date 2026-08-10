```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("protocol P {\n}\n", { language: "avro" });
}

void main();

```
