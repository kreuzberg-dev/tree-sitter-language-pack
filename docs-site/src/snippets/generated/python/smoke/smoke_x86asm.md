---
id: fixture_python_smoke_x86asm
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "x"
    config = {"language": "x86asm"}
    _ = process(source, config)

main()

```
