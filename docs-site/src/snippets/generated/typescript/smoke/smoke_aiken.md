```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("fn main() {\n  1\n}\n", { language: "aiken" });
}

void main();

```
