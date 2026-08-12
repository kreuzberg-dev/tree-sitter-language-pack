---
id: fixture_wasm_data_extraction_nginx_directives
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("worker_processes 4;\nerror_log /var/log/nginx/error.log;\n", { dataExtraction: true, language: "nginx" });
}

void main();

```
