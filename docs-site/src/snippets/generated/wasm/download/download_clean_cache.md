---
id: fixture_wasm_download_clean_cache
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { cleanCache } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = cleanCache();
}

void main();

```
