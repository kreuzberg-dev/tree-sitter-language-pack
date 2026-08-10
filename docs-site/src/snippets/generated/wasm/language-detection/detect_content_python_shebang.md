```typescript title="WebAssembly"
import { detectLanguageFromContent } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = detectLanguageFromContent("#!/usr/bin/env python3\npass");
}

void main();

```
