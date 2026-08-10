```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("console.log('hello');", { language: "javascript" });
}

void main();

```
