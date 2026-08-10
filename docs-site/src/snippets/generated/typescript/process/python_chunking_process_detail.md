```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("def alpha():\n    pass\n\ndef beta():\n    pass\n\ndef gamma():\n    pass\n\ndef delta():\n    pass\n", { chunkMaxSize: 30, language: "python" });
}

void main();

```
