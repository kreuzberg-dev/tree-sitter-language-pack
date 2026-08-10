```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("package main\nfunc main() {}", { language: "go" });
}

void main();

```
