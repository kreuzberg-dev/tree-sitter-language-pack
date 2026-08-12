---
id: fixture_node_data_extraction_csv_single_row
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("x,y,z\n", { dataExtraction: true, language: "csv" });
}

void main();

```
