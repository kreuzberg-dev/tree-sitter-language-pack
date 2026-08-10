```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<p>hi</p>\n", { language: "rshtml" });
}

void main();

```
