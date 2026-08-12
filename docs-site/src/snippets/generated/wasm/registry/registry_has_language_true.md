---
id: fixture_wasm_registry_has_language_true
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { hasLanguage } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = hasLanguage("python");
}

void main();

```
