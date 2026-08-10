```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("---- MODULE Main ----\n====", { language: "tlaplus" });
}

void main();

```
