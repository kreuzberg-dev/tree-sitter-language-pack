```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("def greet(name):\n    \"\"\"Say hello to someone.\"\"\"\n    return f\"Hello {name}\"\n", { docstrings: true, language: "python" });
}

void main();

```
