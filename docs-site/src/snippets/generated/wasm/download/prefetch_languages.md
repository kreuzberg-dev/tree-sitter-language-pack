---
id: fixture_wasm_prefetch_languages
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { prefetch } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = prefetch(["python"]);
}

void main();

```
