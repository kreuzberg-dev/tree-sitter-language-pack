---
id: fixture_node_process_python_imports_detail
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n", { language: "python" });
}

void main();

```
