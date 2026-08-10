```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("actor Main\n  new create(env: Env) => None", { language: "pony" });
}

void main();

```
