```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("#let x = 1", { language: "typst" });
}

void main();

```
