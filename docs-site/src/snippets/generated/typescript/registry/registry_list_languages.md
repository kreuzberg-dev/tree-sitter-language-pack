---
id: fixture_node_registry_list_languages
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { availableLanguages } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = availableLanguages();
}

void main();

```
