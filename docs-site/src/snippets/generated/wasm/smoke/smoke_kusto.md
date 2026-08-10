```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("T | count\n", { language: "kusto" });
}

void main();

```
