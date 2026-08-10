```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("graph TD\nA --> B", { language: "mermaid" });
}

void main();

```
