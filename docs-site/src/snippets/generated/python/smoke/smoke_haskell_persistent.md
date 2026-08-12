---
id: fixture_python_smoke_haskell_persistent
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "Person\n  name String\n"
    config = {"language": "haskell_persistent"}
    _ = process(source, config)

main()

```
