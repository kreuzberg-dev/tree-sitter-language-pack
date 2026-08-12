---
id: fixture_wasm_process_python_symbols
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n", { language: "python", symbols: true });
}

void main();

```
