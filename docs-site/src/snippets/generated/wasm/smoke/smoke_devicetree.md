---
id: fixture_wasm_smoke_devicetree
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("/dts-v1/;\n/ { };", { language: "devicetree" });
}

void main();

```
