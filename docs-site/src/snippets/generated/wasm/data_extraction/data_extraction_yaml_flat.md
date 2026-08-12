---
id: fixture_wasm_data_extraction_yaml_flat
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("host: localhost\nport: 8080\n", { dataExtraction: true, language: "yaml" });
}

void main();

```
