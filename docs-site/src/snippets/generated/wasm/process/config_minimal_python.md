---
id: fixture_wasm_config_minimal_python
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("def hello():\n    pass\n", { language: "python" });
}

void main();

```
