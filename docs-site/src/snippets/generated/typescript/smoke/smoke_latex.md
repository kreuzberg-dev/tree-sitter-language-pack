---
id: fixture_node_smoke_latex
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}", { language: "latex" });
}

void main();

```
