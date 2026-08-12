---
id: fixture_python_smoke_systemtap
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "probe begin {}\n"
    config = {"language": "systemtap"}
    _ = process(source, config)

main()

```
