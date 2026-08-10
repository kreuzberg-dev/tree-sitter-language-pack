```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("msgid \"hello\"\nmsgstr \"world\"", { language: "po" });
}

void main();

```
