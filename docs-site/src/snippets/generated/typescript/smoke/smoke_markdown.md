```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("# Hello\n\nWorld", { language: "markdown" });
}

void main();

```
