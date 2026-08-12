---
id: fixture_wasm_smoke_markdown_inline
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("**bold** and *italic*", { language: "markdown_inline" });
}

void main();

```
