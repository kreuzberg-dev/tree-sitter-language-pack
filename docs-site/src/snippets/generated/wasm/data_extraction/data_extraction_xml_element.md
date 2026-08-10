```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<server id=\"main\"><host>localhost</host></server>", { dataExtraction: true, language: "xml" });
}

void main();

```
