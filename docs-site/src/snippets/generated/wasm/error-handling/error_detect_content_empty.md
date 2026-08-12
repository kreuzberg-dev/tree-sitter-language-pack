---
id: fixture_wasm_error_detect_content_empty
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { detectLanguageFromContent } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = detectLanguageFromContent("");
}

void main();

```
