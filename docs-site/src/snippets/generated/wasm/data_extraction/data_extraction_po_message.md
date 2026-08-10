```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("msgid \"Hello\"\nmsgstr \"Hallo\"\n", { dataExtraction: true, language: "po" });
}

void main();

```
