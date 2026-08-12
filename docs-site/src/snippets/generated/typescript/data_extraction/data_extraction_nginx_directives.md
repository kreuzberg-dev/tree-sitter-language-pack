---
id: fixture_node_data_extraction_nginx_directives
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("worker_processes 4;\nerror_log /var/log/nginx/error.log;\n", { dataExtraction: true, language: "nginx" });
}

void main();

```
