```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("open(\"/x\", O_RDONLY) = 3\n", { language: "strace" });
}

void main();

```
