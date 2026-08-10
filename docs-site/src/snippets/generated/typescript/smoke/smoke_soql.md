```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("SELECT Id FROM Account\n", { language: "soql" });
}

void main();

```
