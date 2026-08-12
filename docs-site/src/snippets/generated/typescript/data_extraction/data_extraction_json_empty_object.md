---
id: fixture_node_data_extraction_json_empty_object
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("{}", { dataExtraction: true, language: "json" });
}

void main();

```
