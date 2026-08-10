```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("SELECT ?s WHERE { ?s ?p ?o }", { language: "sparql" });
}

void main();

```
