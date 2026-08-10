```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("service HelloService {}", { language: "thrift" });
}

void main();

```
