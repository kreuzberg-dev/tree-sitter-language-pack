```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("|0100 LIT 01", { language: "uxntal" });
}

void main();

```
