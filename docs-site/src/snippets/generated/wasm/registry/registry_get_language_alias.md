---
id: fixture_wasm_registry_get_language_alias
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getLanguage } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const language = getLanguage("shell");
}

void main();

```
