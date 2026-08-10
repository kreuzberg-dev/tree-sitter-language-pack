```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("struct Point { x: f64, y: f64 }", { language: "rust" });
}

void main();

```
