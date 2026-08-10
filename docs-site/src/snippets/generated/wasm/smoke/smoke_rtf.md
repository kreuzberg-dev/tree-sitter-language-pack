```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("{\\rtf1 hello}", { language: "rtf" });
}

void main();

```
