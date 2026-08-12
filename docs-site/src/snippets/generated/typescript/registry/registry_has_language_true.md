---
id: fixture_node_registry_has_language_true
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { hasLanguage } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = hasLanguage("python");
}

void main();

```
