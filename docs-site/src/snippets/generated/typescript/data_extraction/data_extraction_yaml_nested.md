---
id: fixture_node_data_extraction_yaml_nested
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("server:\n  host: localhost\n  port: 8080\n", { dataExtraction: true, language: "yaml" });
}

void main();

```
