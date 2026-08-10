```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("package main\nfunc main() {}", { language: "go" });
}

void main();

```
