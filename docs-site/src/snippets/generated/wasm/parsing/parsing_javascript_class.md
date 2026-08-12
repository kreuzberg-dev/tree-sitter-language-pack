---
id: fixture_wasm_parsing_javascript_class
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("class Foo { bar() {} }", { language: "javascript" });
}

void main();

```
