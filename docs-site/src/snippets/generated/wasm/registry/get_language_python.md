---
id: fixture_wasm_get_language_python
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getLanguage } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const language = getLanguage("python");
}

void main();

```
