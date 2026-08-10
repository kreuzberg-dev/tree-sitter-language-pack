```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("def broken(\n    return\nclass", { diagnostics: true, language: "python" });
}

void main();

```
