```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("(defwidget main [] (label :text \"hi\"))", { language: "yuck" });
}

void main();

```
