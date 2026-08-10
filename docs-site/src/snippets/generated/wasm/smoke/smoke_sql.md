```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("SELECT 1;", { language: "sql" });
}

void main();

```
