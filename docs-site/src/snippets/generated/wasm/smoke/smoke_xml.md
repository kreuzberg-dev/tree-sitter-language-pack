```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<?xml version=\"1.0\"?>\n<root>hello</root>", { language: "xml" });
}

void main();

```
