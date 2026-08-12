---
id: fixture_node_data_extraction_cue_fields
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("host: \"localhost\"\nport: 8080\n", { dataExtraction: true, language: "cue" });
}

void main();

```
