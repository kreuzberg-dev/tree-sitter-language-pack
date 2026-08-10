```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("program test.aleo {\n}\n", { language: "leo" });
}

void main();

```
