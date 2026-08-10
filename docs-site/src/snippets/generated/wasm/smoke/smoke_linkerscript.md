```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("SECTIONS { .text : { *(.text) } }", { language: "linkerscript" });
}

void main();

```
