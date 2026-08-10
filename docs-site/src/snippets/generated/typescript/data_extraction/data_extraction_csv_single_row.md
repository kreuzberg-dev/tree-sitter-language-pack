```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("x,y,z\n", { dataExtraction: true, language: "csv" });
}

void main();

```
