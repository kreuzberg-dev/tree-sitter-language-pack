```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("%token EOF\n%%\n", { language: "menhir" });
}

void main();

```
