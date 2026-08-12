---
id: fixture_node_kotlin_package_class_process
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", { language: "kotlin" });
}

void main();

```
