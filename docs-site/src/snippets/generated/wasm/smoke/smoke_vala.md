```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("class Foo {\n}\n", { language: "vala" });
}

void main();

```
