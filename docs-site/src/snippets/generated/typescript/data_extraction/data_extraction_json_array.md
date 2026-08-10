```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("[1, 2, 3]", { dataExtraction: true, language: "json" });
}

void main();

```
