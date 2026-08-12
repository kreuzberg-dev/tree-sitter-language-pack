---
id: fixture_wasm_parsing_rust_struct
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("struct Point { x: f64, y: f64 }", { language: "rust" });
}

void main();

```
