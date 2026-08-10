```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("all:\n\techo hello", { language: "make" });
}

void main();

```
