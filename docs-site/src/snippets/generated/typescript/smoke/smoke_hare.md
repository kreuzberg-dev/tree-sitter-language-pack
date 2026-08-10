```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("export fn main() void = void;", { language: "hare" });
}

void main();

```
