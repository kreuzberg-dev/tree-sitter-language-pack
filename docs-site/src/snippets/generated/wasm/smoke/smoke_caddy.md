```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process(":8080 {\n\trespond \"Hello\"\n}", { language: "caddy" });
}

void main();

```
