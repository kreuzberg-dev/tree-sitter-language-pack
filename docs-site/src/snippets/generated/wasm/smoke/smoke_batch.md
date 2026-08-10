```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("@echo off\necho hello", { language: "batch" });
}

void main();

```
