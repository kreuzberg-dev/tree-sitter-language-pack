```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("BEGIN { }\n", { language: "bpftrace" });
}

void main();

```
