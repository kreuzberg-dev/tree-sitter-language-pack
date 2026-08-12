---
id: fixture_node_data_extraction_json_array
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("[1, 2, 3]", { dataExtraction: true, language: "json" });
}

void main();

```
