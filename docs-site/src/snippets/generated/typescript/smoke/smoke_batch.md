---
id: fixture_node_smoke_batch
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("@echo off\necho hello", { language: "batch" });
}

void main();

```
