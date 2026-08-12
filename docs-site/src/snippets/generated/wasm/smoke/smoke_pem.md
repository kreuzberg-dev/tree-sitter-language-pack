---
id: fixture_wasm_smoke_pem
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----", { language: "pem" });
}

void main();

```
