```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("export function greet(name) {\n  return `Hello ${name}`;\n}\n\nexport const VERSION = '1.0';\n", { language: "javascript" });
}

void main();

```
