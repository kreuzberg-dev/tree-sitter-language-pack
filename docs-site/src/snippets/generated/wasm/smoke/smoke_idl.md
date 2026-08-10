```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("module M {\n};\n", { language: "idl" });
}

void main();

```
