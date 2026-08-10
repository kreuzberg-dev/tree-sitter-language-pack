```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("import QtQuick 2.0\nItem {}", { language: "qmljs" });
}

void main();

```
