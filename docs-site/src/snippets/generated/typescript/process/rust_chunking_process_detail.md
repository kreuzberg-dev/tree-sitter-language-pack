```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("fn alpha() {}\n\nfn beta() {}\n\nfn gamma() {}\n\nfn delta() {}\n", { chunkMaxSize: 30, language: "rust" });
}

void main();

```
