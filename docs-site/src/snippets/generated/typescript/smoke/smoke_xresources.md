```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("*.foreground: #ffffff\n", { language: "xresources" });
}

void main();

```
