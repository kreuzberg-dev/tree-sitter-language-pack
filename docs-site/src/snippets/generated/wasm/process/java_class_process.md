```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("import java.util.List;\n\npublic class Greeter {\n    public String greet(String name) {\n        return \"Hello \" + name;\n    }\n}\n", { language: "java" });
}

void main();

```
