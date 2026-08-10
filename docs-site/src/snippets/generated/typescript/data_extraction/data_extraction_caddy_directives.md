```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("localhost\nroot * /var/www\nfile_server\n", { dataExtraction: true, language: "caddy" });
}

void main();

```
