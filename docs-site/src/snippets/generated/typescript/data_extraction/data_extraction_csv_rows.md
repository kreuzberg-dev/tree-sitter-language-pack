---
id: fixture_node_data_extraction_csv_rows
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("a,b,c\n1,2,3\n", { dataExtraction: true, language: "csv" });
}

void main();

```
