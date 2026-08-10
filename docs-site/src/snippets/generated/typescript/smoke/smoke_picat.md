```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("main => true.\n", { language: "picat" });
}

void main();

```
