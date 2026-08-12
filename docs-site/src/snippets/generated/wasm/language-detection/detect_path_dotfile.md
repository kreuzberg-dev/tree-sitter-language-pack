---
id: fixture_wasm_detect_path_dotfile
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { detectLanguageFromPath } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = detectLanguageFromPath(".gitignore");
}

void main();

```
