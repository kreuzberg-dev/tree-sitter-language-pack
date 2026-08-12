---
id: fixture_wasm_smoke_qmljs
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("import QtQuick 2.0\nItem {}", { language: "qmljs" });
}

void main();

```
