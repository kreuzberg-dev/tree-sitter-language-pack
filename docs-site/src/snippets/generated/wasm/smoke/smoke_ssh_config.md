```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("Host example\n  HostName example.com", { language: "ssh_config" });
}

void main();

```
