```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process(" move.l d0,d1\n", { language: "m68k" });
}

void main();

```
