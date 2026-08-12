---
id: fixture_python_smoke_func
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "() recv_internal() {}"
    config = {"language": "func"}
    _ = process(source, config)

main()

```
