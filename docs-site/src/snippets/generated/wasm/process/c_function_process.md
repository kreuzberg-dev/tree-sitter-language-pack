---
id: fixture_wasm_c_function_process
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("#include <stdio.h>\n\nint main() {\n    printf(\"hello\");\n    return 0;\n}\n", { language: "c" });
}

void main();

```
