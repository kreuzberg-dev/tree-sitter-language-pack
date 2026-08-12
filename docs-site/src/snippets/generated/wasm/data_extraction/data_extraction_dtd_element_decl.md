---
id: fixture_wasm_data_extraction_dtd_element_decl
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<!ELEMENT server (host, port)>\n<!ELEMENT host (#PCDATA)>\n", { dataExtraction: true, language: "dtd" });
}

void main();

```
