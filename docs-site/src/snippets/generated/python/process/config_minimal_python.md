---
id: fixture_python_config_minimal_python
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "def hello():\n    pass\n"
    config = {"language": "python"}
    _ = process(source, config)

main()

```
