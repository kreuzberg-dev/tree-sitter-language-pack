```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<?hh\nfunction main(): void {}", { language: "hack" });
}

void main();

```
