```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("feat: add feature\n\nBody text", { language: "gitcommit" });
}

void main();

```
