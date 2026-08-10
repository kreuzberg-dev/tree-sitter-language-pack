```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("/dts-v1/;\n/ { };", { language: "devicetree" });
}

void main();

```
