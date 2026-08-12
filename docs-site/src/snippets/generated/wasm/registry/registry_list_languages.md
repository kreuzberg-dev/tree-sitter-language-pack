---
id: fixture_wasm_registry_list_languages
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { availableLanguages } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = availableLanguages();
}

void main();

```
