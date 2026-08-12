---
id: fixture_node_parsing_html_element
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<div>hello</div>", { language: "html" });
}

void main();

```
