```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("---@param name string", { language: "luadoc" });
}

void main();

```
