---
id: fixture_node_data_extraction_editorconfig_section
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("[*.rs]\nindent_style = space\nindent_size = 4\n", { dataExtraction: true, language: "editorconfig" });
}

void main();

```
