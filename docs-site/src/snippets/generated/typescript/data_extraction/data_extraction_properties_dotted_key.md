```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("server.host=localhost\nserver.port=8080\n", { dataExtraction: true, language: "properties" });
}

void main();

```
