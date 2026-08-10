```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("def greet(name):\n    return f'Hello, {name}!'\n", { language: "python" });
}

void main();

```
