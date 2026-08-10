```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("def first():\n    x = 1\n    return x\n\ndef second():\n    y = 2\n    return y\n\ndef third():\n    z = 3\n    return z\n", { chunkMaxSize: 50, language: "python" });
}

void main();

```
