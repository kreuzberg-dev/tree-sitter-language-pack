---
id: fixture_wasm_smoke_sparql
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("SELECT ?s WHERE { ?s ?p ?o }", { language: "sparql" });
}

void main();

```
