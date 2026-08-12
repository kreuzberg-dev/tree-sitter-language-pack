---
id: fixture_node_smoke_smali
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process(".class public LMain;\n.super Ljava/lang/Object;", { language: "smali" });
}

void main();

```
