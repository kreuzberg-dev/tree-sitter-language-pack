```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("mov eax, 1", { language: "asm" });
}

void main();

```
