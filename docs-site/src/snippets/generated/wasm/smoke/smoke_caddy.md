---
id: fixture_wasm_smoke_caddy
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process(":8080 {\n\trespond \"Hello\"\n}", { language: "caddy" });
}

void main();

```
