---
id: fixture_wasm_download_multiple_languages
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { download } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = download(["python", "rust"]);
}

void main();

```
