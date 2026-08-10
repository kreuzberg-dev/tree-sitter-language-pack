```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("() recv_internal() {}", { language: "func" });
}

void main();

```
