```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("actor Main\n  new create(env: Env) => None", { language: "pony" });
}

void main();

```
