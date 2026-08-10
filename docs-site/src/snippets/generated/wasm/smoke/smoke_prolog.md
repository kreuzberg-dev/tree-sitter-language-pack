```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("hello :- write('hello'), nl.", { language: "prolog" });
}

void main();

```
