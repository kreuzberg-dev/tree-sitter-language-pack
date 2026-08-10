```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("DESCRIPTION = \"hello\"", { language: "bitbake" });
}

void main();

```
