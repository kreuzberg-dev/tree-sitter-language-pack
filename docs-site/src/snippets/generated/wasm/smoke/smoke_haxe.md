---
id: fixture_wasm_smoke_haxe
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("class Main { static function main() {} }", { language: "haxe" });
}

void main();

```
