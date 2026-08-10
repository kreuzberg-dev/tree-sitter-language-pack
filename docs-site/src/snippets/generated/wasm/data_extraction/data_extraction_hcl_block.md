```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("resource \"aws_instance\" \"web\" {\n  ami = \"ami-123\"\n  instance_type = \"t2.micro\"\n}\n", { dataExtraction: true, language: "hcl" });
}

void main();

```
