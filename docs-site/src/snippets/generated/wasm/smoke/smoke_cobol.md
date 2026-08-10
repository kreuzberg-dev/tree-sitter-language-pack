```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.", { language: "cobol" });
}

void main();

```
