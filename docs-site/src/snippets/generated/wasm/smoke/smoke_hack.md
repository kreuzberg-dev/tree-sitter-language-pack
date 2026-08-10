```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<?hh\nfunction main(): void {}", { language: "hack" });
}

void main();

```
