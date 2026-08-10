```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("class FOO\nend\n", { language: "eiffel" });
}

void main();

```
