```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("def main : IO Unit := pure ()", { language: "lean" });
}

void main();

```
