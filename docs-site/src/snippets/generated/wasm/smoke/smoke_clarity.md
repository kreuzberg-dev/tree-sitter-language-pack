```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("(define-public (hello) (ok true))", { language: "clarity" });
}

void main();

```
