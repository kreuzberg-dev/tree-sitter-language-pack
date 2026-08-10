```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("actor {\n}\n", { language: "motoko" });
}

void main();

```
