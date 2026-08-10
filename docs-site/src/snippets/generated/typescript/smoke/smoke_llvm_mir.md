```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("---\nname: foo\n...\n", { language: "llvm_mir" });
}

void main();

```
