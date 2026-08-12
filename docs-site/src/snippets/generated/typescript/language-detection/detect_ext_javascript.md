---
id: fixture_node_detect_ext_javascript
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { detectLanguageFromExtension } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = detectLanguageFromExtension("js");
}

void main();

```
