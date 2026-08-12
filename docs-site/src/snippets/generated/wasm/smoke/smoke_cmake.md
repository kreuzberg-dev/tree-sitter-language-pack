---
id: fixture_wasm_smoke_cmake
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("cmake_minimum_required(VERSION 3.0)", { language: "cmake" });
}

void main();

```
