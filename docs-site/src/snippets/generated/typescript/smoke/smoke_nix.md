```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("{ pkgs ? import <nixpkgs> {} }: pkgs.hello", { language: "nix" });
}

void main();

```
