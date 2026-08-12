---
id: fixture_wasm_python_error_diagnostics
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("def broken(\n    pass\n", { diagnostics: true, language: "python" });
}

void main();

```
