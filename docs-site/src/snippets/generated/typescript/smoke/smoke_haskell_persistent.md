```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("Person\n  name String\n", { language: "haskell_persistent" });
}

void main();

```
