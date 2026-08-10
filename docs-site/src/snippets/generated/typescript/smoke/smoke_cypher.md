```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("MATCH (n) RETURN n\n", { language: "cypher" });
}

void main();

```
