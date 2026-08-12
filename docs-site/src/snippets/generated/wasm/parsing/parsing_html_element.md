---
id: fixture_wasm_parsing_html_element
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<div>hello</div>", { language: "html" });
}

void main();

```
