---
id: fixture_wasm_smoke_glsl
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("void main() { gl_Position = vec4(0.0); }", { language: "glsl" });
}

void main();

```
