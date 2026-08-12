---
id: fixture_node_smoke_powershell
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("Write-Host 'hello'", { language: "powershell" });
}

void main();

```
