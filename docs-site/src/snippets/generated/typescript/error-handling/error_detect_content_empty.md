---
id: fixture_node_error_detect_content_empty
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { detectLanguageFromContent } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = detectLanguageFromContent("");
}

void main();

```
