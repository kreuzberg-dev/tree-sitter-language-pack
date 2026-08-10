```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("<!ELEMENT server (host, port)>\n<!ELEMENT host (#PCDATA)>\n", { dataExtraction: true, language: "dtd" });
}

void main();

```
