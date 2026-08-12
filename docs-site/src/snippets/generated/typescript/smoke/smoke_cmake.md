---
id: fixture_node_smoke_cmake
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("cmake_minimum_required(VERSION 3.0)", { language: "cmake" });
}

void main();

```
