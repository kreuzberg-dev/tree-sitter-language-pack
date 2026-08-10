```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("*.o\n*.log", { language: "gitignore" });
}

void main();

```
