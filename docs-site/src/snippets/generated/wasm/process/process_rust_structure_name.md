```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("pub struct MyConfig {\n    pub name: String,\n    pub value: i32,\n}\n\nimpl MyConfig {\n    pub fn new() -> Self {\n        Self { name: String::new(), value: 0 }\n    }\n}\n", { language: "rust" });
}

void main();

```
