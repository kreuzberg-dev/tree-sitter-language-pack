---
id: fixture_node_download_configure_custom_dir
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { configure } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = configure({ cacheDir: "/tmp/tslp_test_cache" });
}

void main();

```
