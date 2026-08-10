```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("syntax = \"proto3\";", { language: "proto" });
}

void main();

```
