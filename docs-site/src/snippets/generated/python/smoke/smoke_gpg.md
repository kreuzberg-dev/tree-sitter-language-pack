---
id: fixture_python_smoke_gpg
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
    config = {"language": "gpg"}
    _ = process(source, config)

main()

```
