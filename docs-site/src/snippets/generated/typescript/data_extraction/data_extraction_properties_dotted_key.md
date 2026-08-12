---
id: fixture_node_data_extraction_properties_dotted_key
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("server.host=localhost\nserver.port=8080\n", { dataExtraction: true, language: "properties" });
}

void main();

```
