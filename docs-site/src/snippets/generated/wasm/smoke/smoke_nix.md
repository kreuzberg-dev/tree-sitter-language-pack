```typescript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";
function main() {
  const result = process("{ pkgs ? import <nixpkgs> {} }: pkgs.hello", { language: "nix" });
}

void main();

```
