---
id: fixture_wasm_smoke_wgsl_bevy
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("x", { language: "wgsl_bevy" });
}

void main();

```
