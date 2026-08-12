---
id: fixture_wasm_registry_language_count
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { languageCount } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = languageCount();
}

void main();

```
