```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("type Query { hello: String }", { language: "graphql" });
}

void main();

```
