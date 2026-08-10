```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("export component Foo {}\n", { language: "slint" });
}

void main();

```
