```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("def Hello : Base {}", { language: "tablegen" });
}

void main();

```
