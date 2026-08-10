```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("export void main() {}", { language: "ispc" });
}

void main();

```
