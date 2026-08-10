```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("local x: number = 1", { language: "luau" });
}

void main();

```
