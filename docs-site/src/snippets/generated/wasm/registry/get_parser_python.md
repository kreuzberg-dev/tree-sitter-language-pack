---
id: fixture_wasm_get_parser_python
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getParser } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const parser = getParser("python");
}

void main();

```
