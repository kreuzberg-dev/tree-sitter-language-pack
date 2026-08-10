```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("server {\n  host \"localhost\"\n  port 8080\n}\n", { dataExtraction: true, language: "kdl" });
}

void main();

```
