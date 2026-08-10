```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("Person\n  name String\n", { language: "haskell_persistent" });
}

void main();

```
