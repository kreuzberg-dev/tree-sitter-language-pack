```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("function greet(name: string): string { return `hi ${name}`; }", { language: "typescript" });
}

void main();

```
