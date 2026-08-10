```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process(".class public LMain;\n.super Ljava/lang/Object;", { language: "smali" });
}

void main();

```
