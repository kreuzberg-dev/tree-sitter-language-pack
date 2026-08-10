```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("definition user {}\n", { language: "spicedb" });
}

void main();

```
