```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("[database]\nhost=localhost\nport=5432\n", { dataExtraction: true, language: "ini" });
}

void main();

```
