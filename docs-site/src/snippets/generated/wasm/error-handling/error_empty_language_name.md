```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
try {
    process("hello", { language: "" });
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
