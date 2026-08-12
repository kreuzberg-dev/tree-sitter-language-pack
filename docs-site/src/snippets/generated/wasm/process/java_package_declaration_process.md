---
id: fixture_wasm_java_package_declaration_process
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("package com.example.widget;\n\npublic class Widget {\n    public String name() { return \"w\"; }\n}\n", { language: "java" });
}

void main();

```
