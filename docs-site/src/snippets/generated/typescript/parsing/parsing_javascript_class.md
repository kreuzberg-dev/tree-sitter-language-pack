```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("class Foo { bar() {} }", { language: "javascript" });
}

void main();

```
