```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("export function greet() { return 'hi'; }\nexport const VERSION = '1.0';\nexport default class App {}\n", { language: "javascript" });
}

void main();

```
