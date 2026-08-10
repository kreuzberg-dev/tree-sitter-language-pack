```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("function y = hello(x)\ny = x;\nend", { language: "matlab" });
}

void main();

```
