---
id: fixture_wasm_download_configure_custom_dir
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { configure } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = configure({ cacheDir: "/tmp/tslp_test_cache" });
}

void main();

```
