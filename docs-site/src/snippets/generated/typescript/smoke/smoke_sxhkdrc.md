```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("super + a\n\techo hi\n", { language: "sxhkdrc" });
}

void main();

```
