```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("digraph G { A -> B; }", { language: "dot" });
}

void main();

```
