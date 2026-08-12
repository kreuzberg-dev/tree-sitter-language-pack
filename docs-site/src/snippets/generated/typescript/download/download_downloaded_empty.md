---
id: fixture_node_download_downloaded_empty
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { downloadedLanguages } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = downloadedLanguages();
}

void main();

```
