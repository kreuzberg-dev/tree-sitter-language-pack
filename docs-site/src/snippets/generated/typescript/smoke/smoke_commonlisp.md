```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("(defun hello () (print \"hello\"))", { language: "commonlisp" });
}

void main();

```
