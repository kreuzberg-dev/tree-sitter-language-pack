---
id: fixture_python_smoke_hyprlang
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "general { border_size = 1 }"
    config = {"language": "hyprlang"}
    _ = process(source, config)

main()

```
