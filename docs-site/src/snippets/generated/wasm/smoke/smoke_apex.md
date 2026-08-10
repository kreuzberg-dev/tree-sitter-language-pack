```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("public class Main {}", { language: "apex" });
}

void main();

```
