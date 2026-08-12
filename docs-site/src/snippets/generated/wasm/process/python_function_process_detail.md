---
id: fixture_wasm_python_function_process_detail
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("def greet(name):\n    return f'Hello, {name}!'\n", { language: "python" });
}

void main();

```
