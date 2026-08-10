```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("__global__ void kernel() {}", { language: "cuda" });
}

void main();

```
