---
id: fixture_node_data_extraction_caddy_directives
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("localhost\nroot * /var/www\nfile_server\n", { dataExtraction: true, language: "caddy" });
}

void main();

```
