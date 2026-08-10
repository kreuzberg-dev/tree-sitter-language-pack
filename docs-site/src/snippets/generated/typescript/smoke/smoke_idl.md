```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("module M {\n};\n", { language: "idl" });
}

void main();

```
