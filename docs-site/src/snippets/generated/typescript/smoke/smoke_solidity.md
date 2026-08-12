---
id: fixture_node_smoke_solidity
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("pragma solidity ^0.8.0;\ncontract Main {}", { language: "solidity" });
}

void main();

```
