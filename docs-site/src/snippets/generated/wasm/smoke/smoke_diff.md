```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new", { language: "diff" });
}

void main();

```
