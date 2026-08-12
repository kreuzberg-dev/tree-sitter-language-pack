---
id: fixture_python_smoke_spicedb
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "definition user {}\n"
    config = {"language": "spicedb"}
    _ = process(source, config)

main()

```
