```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("main = putStrLn \"hello\"", { language: "haskell" });
}

void main();

```
