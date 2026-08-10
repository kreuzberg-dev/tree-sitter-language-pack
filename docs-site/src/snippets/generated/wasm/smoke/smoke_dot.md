```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("digraph G { A -> B; }", { language: "dot" });
}

void main();

```
