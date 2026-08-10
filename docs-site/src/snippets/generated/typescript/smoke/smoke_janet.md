```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("(print \"hello\")", { language: "janet" });
}

void main();

```
