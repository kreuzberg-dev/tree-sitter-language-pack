```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("import fs from 'fs';\nimport path from 'path';\n\nfunction process(input) {\n    return input.trim();\n}\n", { language: "javascript" });
}

void main();

```
