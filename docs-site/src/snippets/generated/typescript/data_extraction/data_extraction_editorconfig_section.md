```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("[*.rs]\nindent_style = space\nindent_size = 4\n", { dataExtraction: true, language: "editorconfig" });
}

void main();

```
