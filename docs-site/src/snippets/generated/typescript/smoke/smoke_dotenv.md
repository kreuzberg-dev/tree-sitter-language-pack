```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("KEY=value\n", { language: "dotenv" });
}

void main();

```
