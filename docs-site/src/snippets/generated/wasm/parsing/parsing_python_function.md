---
id: fixture_wasm_parsing_python_function
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("def hello(): pass", { language: "python" });
}

void main();

```
