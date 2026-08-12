---
id: fixture_python_smoke_soql
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "SELECT Id FROM Account\n"
    config = {"language": "soql"}
    _ = process(source, config)

main()

```
