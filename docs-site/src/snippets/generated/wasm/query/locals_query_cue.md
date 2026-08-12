---
id: fixture_wasm_locals_query_cue
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getLocalsQuery } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = getLocalsQuery("cue");
}

void main();

```
