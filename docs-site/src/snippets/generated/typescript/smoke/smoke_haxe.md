---
id: fixture_node_smoke_haxe
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("class Main { static function main() {} }", { language: "haxe" });
}

void main();

```
