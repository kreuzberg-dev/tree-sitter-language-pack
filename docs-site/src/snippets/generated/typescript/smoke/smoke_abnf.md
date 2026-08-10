```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("a = \"b\"\r\n", { language: "abnf" });
}

void main();

```
