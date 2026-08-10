```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("var x:int = 1;", { language: "actionscript" });
}

void main();

```
