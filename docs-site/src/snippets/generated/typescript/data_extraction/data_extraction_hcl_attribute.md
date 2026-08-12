---
id: fixture_node_data_extraction_hcl_attribute
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("region = \"us-east-1\"\ncount  = 3\n", { dataExtraction: true, language: "hcl" });
}

void main();

```
