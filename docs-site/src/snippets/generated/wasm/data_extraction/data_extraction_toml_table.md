```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("[server]\nhost = \"localhost\"\nport = 8080\n", { dataExtraction: true, language: "toml" });
}

void main();

```
