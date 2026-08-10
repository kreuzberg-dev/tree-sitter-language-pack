```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----", { language: "pem" });
}

void main();

```
