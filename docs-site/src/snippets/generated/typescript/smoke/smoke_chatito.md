```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("%[greeting]\n    hello", { language: "chatito" });
}

void main();

```
