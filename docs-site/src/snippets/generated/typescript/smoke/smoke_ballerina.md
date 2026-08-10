```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("public function main() {\n}\n", { language: "ballerina" });
}

void main();

```
