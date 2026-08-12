---
id: fixture_wasm_data_extraction_xml_element
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<server id=\"main\"><host>localhost</host></server>", { dataExtraction: true, language: "xml" });
}

void main();

```
