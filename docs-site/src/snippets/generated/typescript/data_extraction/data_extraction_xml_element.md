---
id: fixture_node_data_extraction_xml_element
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<server id=\"main\"><host>localhost</host></server>", { dataExtraction: true, language: "xml" });
}

void main();

```
