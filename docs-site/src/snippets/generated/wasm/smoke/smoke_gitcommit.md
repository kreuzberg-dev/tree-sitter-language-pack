```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("feat: add feature\n\nBody text", { language: "gitcommit" });
}

void main();

```
