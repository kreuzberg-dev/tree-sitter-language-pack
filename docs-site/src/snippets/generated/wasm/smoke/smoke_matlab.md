```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("function y = hello(x)\ny = x;\nend", { language: "matlab" });
}

void main();

```
