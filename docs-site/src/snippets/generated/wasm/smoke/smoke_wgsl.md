```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }", { language: "wgsl" });
}

void main();

```
