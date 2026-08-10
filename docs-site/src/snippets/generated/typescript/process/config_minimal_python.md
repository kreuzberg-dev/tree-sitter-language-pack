```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("def hello():\n    pass\n", { language: "python" });
}

void main();

```
