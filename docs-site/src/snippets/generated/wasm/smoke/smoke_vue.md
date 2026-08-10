```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("<template><div>hello</div></template>", { language: "vue" });
}

void main();

```
