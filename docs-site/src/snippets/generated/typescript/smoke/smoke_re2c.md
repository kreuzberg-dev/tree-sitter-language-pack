```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("/*!re2c\n  [a-z]+ { return; }\n*/", { language: "re2c" });
}

void main();

```
