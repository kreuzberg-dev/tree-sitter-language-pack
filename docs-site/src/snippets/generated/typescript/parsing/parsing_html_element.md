```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<div>hello</div>", { language: "html" });
}

void main();

```
