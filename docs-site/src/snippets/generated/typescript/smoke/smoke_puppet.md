```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("notify { 'hello': }", { language: "puppet" });
}

void main();

```
