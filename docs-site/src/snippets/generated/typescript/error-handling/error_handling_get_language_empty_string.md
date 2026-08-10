```typescript title="TypeScript"
import { getLanguage } from "@xberg-io/tree-sitter-language-pack";
function main() {
try {
    getLanguage("");
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
