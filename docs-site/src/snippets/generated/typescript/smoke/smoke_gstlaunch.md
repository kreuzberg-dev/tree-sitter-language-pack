```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("fakesrc ! fakesink", { language: "gstlaunch" });
}

void main();

```
