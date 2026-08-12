---
id: fixture_python_smoke_asm
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "mov eax, 1"
    config = {"language": "asm"}
    _ = process(source, config)

main()

```
