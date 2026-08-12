---
id: fixture_python_smoke_llvm
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "define i32 @main() { ret i32 0 }"
    config = {"language": "llvm"}
    _ = process(source, config)

main()

```
