```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("set x to 1\n", { language: "applescript" });
}

void main();

```
