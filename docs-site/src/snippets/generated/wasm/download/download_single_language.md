---
id: fixture_wasm_download_single_language
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { download } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = download(["python"]);
}

void main();

```
