```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("@startuml\n@enduml\n", { language: "plantuml" });
}

void main();

```
