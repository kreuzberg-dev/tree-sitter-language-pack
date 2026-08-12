---
id: fixture_wasm_download_manifest_languages
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { manifestLanguages } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = manifestLanguages();
}

void main();

```
