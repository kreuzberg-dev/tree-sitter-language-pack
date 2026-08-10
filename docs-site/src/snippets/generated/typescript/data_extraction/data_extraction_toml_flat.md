```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("host = \"localhost\"\nport = 8080\n", { dataExtraction: true, language: "toml" });
}

void main();

```
