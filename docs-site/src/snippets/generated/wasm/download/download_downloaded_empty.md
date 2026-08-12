---
id: fixture_wasm_download_downloaded_empty
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { downloadedLanguages } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = downloadedLanguages();
}

void main();

```
