---
id: fixture_node_smoke_qmljs
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("import QtQuick 2.0\nItem {}", { language: "qmljs" });
}

void main();

```
