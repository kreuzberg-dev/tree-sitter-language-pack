```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----", { language: "pem" });
}

void main();

```
