```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("data _null_;\nrun;\n", { language: "sas" });
}

void main();

```
