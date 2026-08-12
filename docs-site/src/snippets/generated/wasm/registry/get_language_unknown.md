---
id: fixture_wasm_get_language_unknown
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { getLanguage } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
try {
    getLanguage("nonexistent_xyz");
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
