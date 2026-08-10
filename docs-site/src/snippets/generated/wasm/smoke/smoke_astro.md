```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("---\n---\n<p>hello</p>", { language: "astro" });
}

void main();

```
