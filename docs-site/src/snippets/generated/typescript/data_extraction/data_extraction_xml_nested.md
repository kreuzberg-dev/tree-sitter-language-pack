---
id: fixture_node_data_extraction_xml_nested
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<config><host>localhost</host><port>8080</port></config>", { dataExtraction: true, language: "xml" });
}

void main();

```
