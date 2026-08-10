```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<br/>", { dataExtraction: true, language: "xml" });
}

void main();

```
