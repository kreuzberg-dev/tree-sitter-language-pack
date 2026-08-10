```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("(defwidget main [] (label :text \"hi\"))", { language: "yuck" });
}

void main();

```
