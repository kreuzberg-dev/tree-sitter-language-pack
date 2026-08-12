---
id: fixture_node_error_handling_get_language_empty_string
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { getLanguage } from "@xberg-io/tree-sitter-language-pack";
function main() {
try {
    getLanguage("");
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
