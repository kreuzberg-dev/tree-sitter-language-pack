---
id: fixture_wasm_parsing_typescript_function
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("function greet(name: string): string { return `hi ${name}`; }", { language: "typescript" });
}

void main();

```
