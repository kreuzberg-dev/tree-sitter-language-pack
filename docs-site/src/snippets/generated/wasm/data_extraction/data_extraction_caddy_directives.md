```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("localhost\nroot * /var/www\nfile_server\n", { dataExtraction: true, language: "caddy" });
}

void main();

```
