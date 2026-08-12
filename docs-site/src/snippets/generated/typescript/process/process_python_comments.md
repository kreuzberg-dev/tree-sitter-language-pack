---
id: fixture_node_process_python_comments
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n", { comments: true, language: "python" });
}

void main();

```
