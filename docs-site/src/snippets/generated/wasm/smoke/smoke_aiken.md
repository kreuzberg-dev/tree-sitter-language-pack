```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("fn main() {\n  1\n}\n", { language: "aiken" });
}

void main();

```
