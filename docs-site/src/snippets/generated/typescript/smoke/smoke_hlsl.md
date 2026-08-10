```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("float4 main() : SV_Target { return 0; }", { language: "hlsl" });
}

void main();

```
