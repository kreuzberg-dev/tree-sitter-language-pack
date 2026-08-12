---
id: fixture_wasm_smoke_smali
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process(".class public LMain;\n.super Ljava/lang/Object;", { language: "smali" });
}

void main();

```
