```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<!ELEMENT note (body)>", { language: "dtd" });
}

void main();

```
