---
id: fixture_wasm_process_python_comments
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n", { comments: true, language: "python" });
}

void main();

```
