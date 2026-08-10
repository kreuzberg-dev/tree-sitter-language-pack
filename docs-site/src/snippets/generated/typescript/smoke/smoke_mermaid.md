```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("graph TD\nA --> B", { language: "mermaid" });
}

void main();

```
