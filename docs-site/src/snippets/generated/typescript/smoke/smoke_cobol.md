---
id: fixture_node_smoke_cobol
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.", { language: "cobol" });
}

void main();

```
