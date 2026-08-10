```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("[*.rs]\nindent_style = space\nindent_size = 4\n", { dataExtraction: true, language: "editorconfig" });
}

void main();

```
