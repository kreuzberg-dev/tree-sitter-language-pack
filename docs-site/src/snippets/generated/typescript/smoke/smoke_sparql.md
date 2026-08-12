---
id: fixture_node_smoke_sparql
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("SELECT ?s WHERE { ?s ?p ?o }", { language: "sparql" });
}

void main();

```
