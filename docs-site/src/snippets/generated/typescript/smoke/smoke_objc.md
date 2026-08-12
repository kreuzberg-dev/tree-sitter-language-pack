---
id: fixture_node_smoke_objc
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("@interface Main @end", { language: "objc" });
}

void main();

```
