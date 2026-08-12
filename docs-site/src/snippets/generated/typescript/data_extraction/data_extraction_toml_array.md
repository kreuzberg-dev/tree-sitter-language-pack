---
id: fixture_node_data_extraction_toml_array
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("ports = [8080, 8081, 8082]\n", { dataExtraction: true, language: "toml" });
}

void main();

```
