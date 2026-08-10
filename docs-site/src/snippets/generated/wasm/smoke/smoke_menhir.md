```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("%token EOF\n%%\n", { language: "menhir" });
}

void main();

```
