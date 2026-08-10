```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("pub fn main() { }", { language: "gleam" });
}

void main();

```
