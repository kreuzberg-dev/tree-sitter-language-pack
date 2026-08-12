---
id: fixture_node_detect_path_java_root
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { detectLanguageFromPath } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = detectLanguageFromPath("Main.java");
}

void main();

```
