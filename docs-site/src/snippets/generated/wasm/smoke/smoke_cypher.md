```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("MATCH (n) RETURN n\n", { language: "cypher" });
}

void main();

```
