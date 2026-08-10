```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("_method object.hello\n_endmethod", { language: "magik" });
}

void main();

```
