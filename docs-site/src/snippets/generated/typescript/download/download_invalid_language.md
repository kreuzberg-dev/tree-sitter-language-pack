```typescript title="TypeScript"
import { download } from "@xberg-io/tree-sitter-language-pack";
function main() {
try {
    download(["zzz_definitely_not_a_real_language_xyz"]);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
