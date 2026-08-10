```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("float4 main() : SV_Target { return 0; }", { language: "hlsl" });
}

void main();

```
