```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}", { language: "latex" });
}

void main();

```
