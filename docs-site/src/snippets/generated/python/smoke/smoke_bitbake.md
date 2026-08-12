---
id: fixture_python_smoke_bitbake
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'DESCRIPTION = "hello"'
    config = {"language": "bitbake"}
    _ = process(source, config)

main()

```
