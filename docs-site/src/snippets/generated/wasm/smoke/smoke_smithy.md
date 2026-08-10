```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("namespace example\nstring MyString", { language: "smithy" });
}

void main();

```
