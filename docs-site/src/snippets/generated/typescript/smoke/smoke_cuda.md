```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("__global__ void kernel() {}", { language: "cuda" });
}

void main();

```
