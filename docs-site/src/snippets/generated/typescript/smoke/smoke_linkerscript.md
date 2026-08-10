```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("SECTIONS { .text : { *(.text) } }", { language: "linkerscript" });
}

void main();

```
