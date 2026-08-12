---
id: fixture_node_detect_ext_gherkin
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { detectLanguageFromExtension } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = detectLanguageFromExtension("feature");
}

void main();

```
