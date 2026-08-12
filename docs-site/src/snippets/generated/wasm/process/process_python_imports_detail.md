---
id: fixture_wasm_process_python_imports_detail
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n", { language: "python" });
}

void main();

```
