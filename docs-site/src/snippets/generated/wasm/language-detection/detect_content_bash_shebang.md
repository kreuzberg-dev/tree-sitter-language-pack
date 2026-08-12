---
id: fixture_wasm_detect_content_bash_shebang
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { detectLanguageFromContent } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = detectLanguageFromContent("#!/bin/bash\necho hi");
}

void main();

```
