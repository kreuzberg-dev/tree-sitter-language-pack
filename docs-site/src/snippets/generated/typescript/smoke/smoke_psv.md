```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("a|b|c\n1|2|3", { language: "psv" });
}

void main();

```
