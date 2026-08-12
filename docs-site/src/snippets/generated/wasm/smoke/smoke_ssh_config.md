---
id: fixture_wasm_smoke_ssh_config
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("Host example\n  HostName example.com", { language: "ssh_config" });
}

void main();

```
