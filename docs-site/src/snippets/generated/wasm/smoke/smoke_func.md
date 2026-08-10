```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("() recv_internal() {}", { language: "func" });
}

void main();

```
