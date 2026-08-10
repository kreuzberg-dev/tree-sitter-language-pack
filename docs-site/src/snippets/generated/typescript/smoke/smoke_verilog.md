```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("module main; endmodule", { language: "verilog" });
}

void main();

```
