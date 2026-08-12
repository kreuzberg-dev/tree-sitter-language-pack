---
id: fixture_wasm_smoke_solidity
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("pragma solidity ^0.8.0;\ncontract Main {}", { language: "solidity" });
}

void main();

```
