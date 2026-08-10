```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<div>hello</div>", { language: "html" });
}

void main();

```
