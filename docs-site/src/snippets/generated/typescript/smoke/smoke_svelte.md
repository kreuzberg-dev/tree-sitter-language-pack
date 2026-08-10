```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<script>let x = 1;</script>", { language: "svelte" });
}

void main();

```
