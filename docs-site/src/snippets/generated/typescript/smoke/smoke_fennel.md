```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("(fn hello [] (print :hello))", { language: "fennel" });
}

void main();

```
