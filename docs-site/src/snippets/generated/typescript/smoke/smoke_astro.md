```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("---\n---\n<p>hello</p>", { language: "astro" });
}

void main();

```
