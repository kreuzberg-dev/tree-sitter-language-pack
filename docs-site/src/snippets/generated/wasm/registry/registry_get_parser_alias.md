---
id: fixture_wasm_registry_get_parser_alias
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getParser } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const parser = getParser("shell");
}

void main();

```
