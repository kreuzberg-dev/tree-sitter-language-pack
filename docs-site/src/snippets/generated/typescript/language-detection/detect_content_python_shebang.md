---
id: fixture_node_detect_content_python_shebang
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { detectLanguageFromContent } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = detectLanguageFromContent("#!/usr/bin/env python3\npass");
}

void main();

```
