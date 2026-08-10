```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("msgid \"Hello\"\nmsgstr \"Hallo\"\n", { dataExtraction: true, language: "po" });
}

void main();

```
