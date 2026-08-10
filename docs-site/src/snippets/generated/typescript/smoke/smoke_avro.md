```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("protocol P {\n}\n", { language: "avro" });
}

void main();

```
