```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<!ELEMENT note (body)>", { language: "dtd" });
}

void main();

```
