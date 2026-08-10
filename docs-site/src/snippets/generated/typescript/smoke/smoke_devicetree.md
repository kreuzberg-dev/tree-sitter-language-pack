```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("/dts-v1/;\n/ { };", { language: "devicetree" });
}

void main();

```
