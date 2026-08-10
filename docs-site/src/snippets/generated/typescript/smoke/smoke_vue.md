```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<template><div>hello</div></template>", { language: "vue" });
}

void main();

```
