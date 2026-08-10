```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("include *.txt", { language: "pymanifest" });
}

void main();

```
