```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("def broken(\n    pass\n", { diagnostics: true, language: "python" });
}

void main();

```
