---
id: fixture_wasm_detect_ext_java
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { detectLanguageFromExtension } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = detectLanguageFromExtension("java");
}

void main();

```
