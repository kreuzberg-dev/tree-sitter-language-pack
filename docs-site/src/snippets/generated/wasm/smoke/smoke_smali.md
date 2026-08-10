```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process(".class public LMain;\n.super Ljava/lang/Object;", { language: "smali" });
}

void main();

```
