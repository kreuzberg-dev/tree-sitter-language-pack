```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("group(\"hello\") {}", { language: "gn" });
}

void main();

```
