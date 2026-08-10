```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("IO.puts(\"hello\")", { language: "elixir" });
}

void main();

```
