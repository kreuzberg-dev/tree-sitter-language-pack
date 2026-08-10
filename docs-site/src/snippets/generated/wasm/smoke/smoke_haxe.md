```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("class Main { static function main() {} }", { language: "haxe" });
}

void main();

```
