```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("cmake_minimum_required(VERSION 3.0)", { language: "cmake" });
}

void main();

```
