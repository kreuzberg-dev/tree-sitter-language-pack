```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("server:\n  host: localhost\n  port: 8080\n", { dataExtraction: true, language: "yaml" });
}

void main();

```
