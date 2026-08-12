---
id: fixture_wasm_data_extraction_hcl_attribute
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("region = \"us-east-1\"\ncount  = 3\n", { dataExtraction: true, language: "hcl" });
}

void main();

```
