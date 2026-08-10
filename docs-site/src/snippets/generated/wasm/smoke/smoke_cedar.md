```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("permit(principal, action, resource);", { language: "cedar" });
}

void main();

```
