```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("# A comment\ndef greet(name):\n    \"\"\"Say hello.\"\"\"\n    return f'Hi {name}'\n\nimport os\n", { language: "python" });
}

void main();

```
