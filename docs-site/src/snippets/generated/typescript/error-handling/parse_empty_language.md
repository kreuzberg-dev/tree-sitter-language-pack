```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
try {
    process("x = 1", { language: "" });
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
