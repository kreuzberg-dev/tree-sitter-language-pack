```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<?php echo 'hello'; ?>", { language: "php" });
}

void main();

```
