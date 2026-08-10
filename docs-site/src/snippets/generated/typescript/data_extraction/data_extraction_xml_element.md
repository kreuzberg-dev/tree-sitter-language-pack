```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<server id=\"main\"><host>localhost</host></server>", { dataExtraction: true, language: "xml" });
}

void main();

```
