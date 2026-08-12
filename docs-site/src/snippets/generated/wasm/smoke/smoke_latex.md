---
id: fixture_wasm_smoke_latex
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}", { language: "latex" });
}

void main();

```
