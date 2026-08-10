```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("===========\nTest\n===========\n---\n(node)", { language: "test" });
}

void main();

```
