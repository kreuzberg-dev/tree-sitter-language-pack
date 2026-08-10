```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("SELECT 1;\n", { language: "postgres" });
}

void main();

```
