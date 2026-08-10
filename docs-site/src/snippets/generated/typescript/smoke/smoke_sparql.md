```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("SELECT ?s WHERE { ?s ?p ?o }", { language: "sparql" });
}

void main();

```
