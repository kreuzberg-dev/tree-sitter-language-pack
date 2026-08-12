---
id: fixture_wasm_smoke_haml
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("%p hello\n", { language: "haml" });
}

void main();

```
