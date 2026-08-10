```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("int main() { return 0; }", { language: "cpp" });
}

void main();

```
