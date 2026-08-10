```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("fun main()\n  1\n", { language: "koka" });
}

void main();

```
