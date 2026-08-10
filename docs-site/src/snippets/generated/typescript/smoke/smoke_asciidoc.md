```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("= Title\n\nParagraph.", { language: "asciidoc" });
}

void main();

```
