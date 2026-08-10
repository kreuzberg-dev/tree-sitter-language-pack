```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("table Foo {}\n", { language: "flatbuffers" });
}

void main();

```
