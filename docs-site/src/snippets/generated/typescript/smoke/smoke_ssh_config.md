---
id: fixture_node_smoke_ssh_config
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("Host example\n  HostName example.com", { language: "ssh_config" });
}

void main();

```
