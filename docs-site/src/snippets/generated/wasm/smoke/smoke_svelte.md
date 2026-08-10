```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<script>let x = 1;</script>", { language: "svelte" });
}

void main();

```
