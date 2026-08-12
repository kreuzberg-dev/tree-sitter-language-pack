---
id: fixture_wasm_parsing_go_function
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("package main\nfunc main() {}", { language: "go" });
}

void main();

```
