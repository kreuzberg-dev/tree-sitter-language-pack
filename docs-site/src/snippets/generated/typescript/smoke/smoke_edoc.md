```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("@doc foo\n", { language: "edoc" });
}

void main();

```
