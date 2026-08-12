---
id: fixture_node_process_javascript_exports_count
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("export function greet() { return 'hi'; }\nexport const VERSION = '1.0';\nexport default class App {}\n", { language: "javascript" });
}

void main();

```
