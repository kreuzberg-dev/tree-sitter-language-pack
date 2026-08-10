```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("const x: number = 42;", { language: "typescript" });
}

void main();

```
