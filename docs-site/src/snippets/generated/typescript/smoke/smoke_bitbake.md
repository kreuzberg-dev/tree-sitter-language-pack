```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("DESCRIPTION = \"hello\"", { language: "bitbake" });
}

void main();

```
