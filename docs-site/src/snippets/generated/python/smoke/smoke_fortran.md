---
id: fixture_python_smoke_fortran
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "program main\nend program main"
    config = {"language": "fortran"}
    _ = process(source, config)

main()

```
