---
id: fixture_wasm_data_extraction_xml_nested
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<config><host>localhost</host><port>8080</port></config>", { dataExtraction: true, language: "xml" });
}

void main();

```
