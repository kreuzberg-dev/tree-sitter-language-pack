```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("extends Node\nfunc _ready():\n\tpass", { language: "gdscript" });
}

void main();

```
