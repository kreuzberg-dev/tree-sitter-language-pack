```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("pub fn main() void {}", { language: "zig" });
}

void main();

```
