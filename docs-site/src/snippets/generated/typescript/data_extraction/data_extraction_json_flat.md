```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("{\"host\": \"localhost\", \"port\": 8080}", { dataExtraction: true, language: "json" });
}

void main();

```
