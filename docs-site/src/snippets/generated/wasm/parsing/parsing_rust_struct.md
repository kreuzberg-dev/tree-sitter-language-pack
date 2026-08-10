```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("struct Point { x: f64, y: f64 }", { language: "rust" });
}

void main();

```
