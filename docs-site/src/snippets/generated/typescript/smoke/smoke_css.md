```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("body { color: red; }", { language: "css" });
}

void main();

```
