```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("Hello\n=====\n\nWorld", { language: "rst" });
}

void main();

```
