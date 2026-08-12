---
id: fixture_wasm_detect_content_no_shebang
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { detectLanguageFromContent } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = detectLanguageFromContent("no shebang here");
}

void main();

```
