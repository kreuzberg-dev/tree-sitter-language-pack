```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("init {\n}\n", { language: "promela" });
}

void main();

```
