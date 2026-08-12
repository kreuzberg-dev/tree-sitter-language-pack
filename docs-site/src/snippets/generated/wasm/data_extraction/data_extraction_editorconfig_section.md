---
id: fixture_wasm_data_extraction_editorconfig_section
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("[*.rs]\nindent_style = space\nindent_size = 4\n", { dataExtraction: true, language: "editorconfig" });
}

void main();

```
