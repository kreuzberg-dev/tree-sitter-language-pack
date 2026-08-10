```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<p>hi</p>\n", { language: "rshtml" });
}

void main();

```
