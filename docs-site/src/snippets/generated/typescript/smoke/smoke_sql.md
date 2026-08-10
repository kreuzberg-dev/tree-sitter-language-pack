```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("SELECT 1;", { language: "sql" });
}

void main();

```
