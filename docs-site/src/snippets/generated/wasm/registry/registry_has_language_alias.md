---
id: fixture_wasm_registry_has_language_alias
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { hasLanguage } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = hasLanguage("shell");
}

void main();

```
