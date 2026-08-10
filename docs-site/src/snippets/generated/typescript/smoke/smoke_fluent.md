```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("hello = Hello\n", { language: "fluent" });
}

void main();

```
