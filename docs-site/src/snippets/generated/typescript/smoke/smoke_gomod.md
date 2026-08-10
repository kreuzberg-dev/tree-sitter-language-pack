```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("module example.com/hello\n\ngo 1.21", { language: "gomod" });
}

void main();

```
