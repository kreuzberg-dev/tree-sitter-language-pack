---
id: fixture_wasm_download_cache_dir
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { cacheDir } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = cacheDir();
}

void main();

```
