```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("void main() { gl_Position = vec4(0.0); }", { language: "glsl" });
}

void main();

```
