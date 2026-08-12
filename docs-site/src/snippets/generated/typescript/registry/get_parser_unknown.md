---
id: fixture_node_get_parser_unknown
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { getParser } from "@xberg-io/tree-sitter-language-pack";
function main() {
try {
    getParser("nonexistent_xyz");
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
