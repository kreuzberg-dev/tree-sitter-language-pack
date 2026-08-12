---
id: fixture_node_data_extraction_hjson_flat
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("{\n  host: \"localhost\"\n  port: 8080\n}\n", { dataExtraction: true, language: "hjson" });
}

void main();

```
