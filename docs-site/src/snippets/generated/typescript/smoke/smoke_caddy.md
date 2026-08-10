```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process(":8080 {\n\trespond \"Hello\"\n}", { language: "caddy" });
}

void main();

```
