```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("{\n  host: \"localhost\"\n  port: 8080\n}\n", { dataExtraction: true, language: "hjson" });
}

void main();

```
