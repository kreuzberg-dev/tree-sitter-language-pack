```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", { language: "kotlin" });
}

void main();

```
