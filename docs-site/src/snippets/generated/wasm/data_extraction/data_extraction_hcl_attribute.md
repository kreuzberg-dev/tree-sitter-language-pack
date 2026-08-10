```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("region = \"us-east-1\"\ncount  = 3\n", { dataExtraction: true, language: "hcl" });
}

void main();

```
