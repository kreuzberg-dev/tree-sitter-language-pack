---
id: fixture_node_smoke_xml
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<?xml version=\"1.0\"?>\n<root>hello</root>", { language: "xml" });
}

void main();

```
